use rusqlite::{Connection, backup::Backup};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const KEY: [u8; 32] = [0x42; 32];
const WRONG_KEY: [u8; 32] = [0x24; 32];
const MARKER: &str = "DCMS_SQLCIPHER_PROOF_SENTINEL_NOT_PLAINTEXT";

struct TempDirectory(PathBuf);

impl TempDirectory {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "dcms-phase1-sqlcipher-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create isolated proof directory");
        Self(path)
    }

    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(DIGITS[(byte >> 4) as usize] as char);
        encoded.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn assert_marker_is_not_plaintext(directory: &Path) {
    for entry in fs::read_dir(directory).expect("list proof files") {
        let bytes = fs::read(entry.expect("read entry").path()).expect("read encrypted file");
        assert!(
            !bytes.windows(MARKER.len()).any(|window| window == MARKER.as_bytes()),
            "plaintext marker leaked into a database, WAL, SHM, or backup file"
        );
    }
}

fn open_keyed(path: &Path, key: &[u8; 32]) -> rusqlite::Result<Connection> {
    let connection = Connection::open(path)?;
    // SQLCipher's raw-key form: the bytes are fixed test data, hex-encoded by this harness.
    connection.execute_batch(&format!(r#"PRAGMA key = "x'{}'";"#, hex(key)))?;
    let cipher_version: String = connection.query_row("PRAGMA cipher_version", [], |row| row.get(0))?;
    assert!(!cipher_version.trim().is_empty(), "SQLCipher was not linked");
    Ok(connection)
}

#[test]
fn windows_sqlcipher_wrong_key_ciphertext_and_online_backup_round_trip() {
    let directory = TempDirectory::new();
    let source_path = directory.file("source.db");
    let backup_path = directory.file("backup.db");

    {
        let source = open_keyed(&source_path, &KEY).expect("open keyed source");
        source
            .execute_batch(
                "PRAGMA journal_mode=WAL;
                 CREATE TABLE proof_records (id INTEGER PRIMARY KEY, marker TEXT NOT NULL);",
            )
            .expect("initialize encrypted source");
        source
            .execute("INSERT INTO proof_records(marker) VALUES (?1)", [MARKER])
            .expect("insert synthetic marker");
        source
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .expect("checkpoint encrypted source");

        let returned: String = source
            .query_row("SELECT marker FROM proof_records WHERE id = 1", [], |row| row.get(0))
            .expect("read keyed marker");
        assert_eq!(returned, MARKER);
    }

    assert_marker_is_not_plaintext(&directory.0);

    {
        let wrong_key = open_keyed(&source_path, &WRONG_KEY).expect("open wrong-key handle");
        let result = wrong_key.query_row("SELECT count(*) FROM proof_records", [], |row| {
            row.get::<_, i64>(0)
        });
        assert!(result.is_err(), "wrong SQLCipher key unexpectedly opened the schema");
    }

    {
        let source = open_keyed(&source_path, &KEY).expect("reopen keyed source");
        let mut destination = open_keyed(&backup_path, &KEY).expect("open keyed destination");
        destination
            .execute_batch(
                "CREATE TABLE destination_bootstrap (id INTEGER PRIMARY KEY);
                 DROP TABLE destination_bootstrap;",
            )
            .expect("initialize encrypted backup destination");
        {
            let backup = Backup::new(&source, &mut destination).expect("initialize online backup");
            backup
                .run_to_completion(16, Duration::from_millis(25), None)
                .expect("copy encrypted database via SQLite Online Backup API");
        }
        let copied: String = destination
            .query_row("SELECT marker FROM proof_records WHERE id = 1", [], |row| row.get(0))
            .expect("read online-backup result");
        assert_eq!(copied, MARKER);
    }

    assert_marker_is_not_plaintext(&directory.0);

    {
        let wrong_key = open_keyed(&backup_path, &WRONG_KEY).expect("open wrong-key backup handle");
        let result = wrong_key.query_row("SELECT count(*) FROM proof_records", [], |row| {
            row.get::<_, i64>(0)
        });
        assert!(result.is_err(), "wrong key unexpectedly opened online-backup result");
    }
}
