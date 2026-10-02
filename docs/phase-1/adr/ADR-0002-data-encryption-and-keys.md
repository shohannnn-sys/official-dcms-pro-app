# ADR-0002 — Local encryption and key protection

- **Status:** Provisional; blocked on exact Windows SQLCipher/DPAPI/recovery evidence and an owner-approved recovery UX.
- **Date:** 2026-10-02 (UTC)
- **Decision owner:** Product owner
- **Related decisions:** ADR-0001 (client boundary), ADR-0004 (portable backup)

## Context

The clinic database contains identifiable patient, clinical, appointment, staff and financial data. Ordinary app-level RBAC does not protect a copied database, an unlocked Windows profile, local malware or an administrator. The product is offline-first; no hosted key service or cloud KMS is allowed. The Phase 0 design therefore requires database encryption and a recoverable portable key path rather than a silent plaintext fallback.

The Rust/Rusqlite path exposes a bundled SQLCipher Community Edition build candidate. Microsoft documents an unofficial open-source `SQLitePCLRaw.bundle_e_sqlcipher` for .NET and marks that NuGet package legacy/unmaintained; Zetetic's supported .NET integrations are Commercial/Enterprise. This evidence favors the Rust/Tauri candidate but does not prove a Windows build or license audit.

## Proposed decision

### Cryptographic and storage boundary

- Use a cryptographically random 256-bit database key `K_db`, generated at first authorized setup. It is not derived from an owner password or activation code.
- Store only a Windows **DPAPI CurrentUser-scope** protected copy in the designated user's app-data security directory, with a versioned envelope and purpose-specific optional entropy. Do not request `CRYPTPROTECT_LOCAL_MACHINE`. Use UI-forbidden DPAPI calls so unlock/backup never unexpectedly prompt from a background task.
- On each SQLCipher connection, provide the database key before any schema/query operation, verify `cipher_version` is the expected implementation, and perform a keyed schema read before treating the connection as usable. Use the same pinned cipher profile for the active DB, the online-backup destination and restore verification.
- The exact SQLCipher key representation/API (32-byte raw key vs a high-entropy passphrase), PRAGMA/FFI initialization method, page size, KDF settings and rusqlite features must be selected in the Windows proof. If raw-key syntax is used, construct it only from generated fixed-length hex, never from UI text; ensure the SQLCipher raw-key form is actually used rather than accidentally being interpreted as a passphrase. Zeroize temporary key buffers as far as the libraries permit.
- Use an explicit `rusqlite` bundled-SQLCipher + vendored-OpenSSL build, subject to current version, FIPS/crypto statements not being overclaimed, MSVC proof and full license notices. Pin all versions in a lockfile before implementation.
- Use a separate random attachment encryption key stored only as encrypted database key material. Encrypt managed attachment bytes with an authenticated cipher (candidate AES-256-GCM), unique random nonce per object, and opaque object ID as authenticated associated data. Keep the database row, cipher version, nonce and content hash; reject mismatched IDs/tags. A copied attachment directory alone must not reveal its content. Test key rotation and interrupted re-encryption before claiming it.
- The SQLCipher database and its WAL/journal are treated as one protected local data set. Use local NTFS app-data, restrictive Windows ACLs, WAL only after Windows verification, serialized writes, bounded readers, checkpoint/snapshot behavior and a single-instance/maintenance lock. Never copy only the live main DB file as a backup.

### Key hierarchy and lifecycle

```text
Windows CurrentUser DPAPI
        │ unwraps
        ▼
K_db (random 256-bit; SQLCipher page key)
        ├── SQLCipher database + WAL/journal
        │       └── random attachment key material (inside encrypted DB)
        └── wrapped for offline recovery recipient inside each backup

Age X25519 recovery identity (user-held private key; not stored by the app)
        └── encrypts/decrypts portable backup envelope and wrapped K_db
```

- App login passwords are stored as salted Argon2id verifiers in the SQLCipher DB; they never become the database encryption key.
- `K_db` is unwrapped only in process memory when opening the DB. Do not log it, include it in diagnostics, serialize it to IPC or pass it to the renderer.
- Lock and process exit clear accessible UI state and drop/zeroize owned key buffers where feasible. This is defense-in-depth, not a guarantee that all copies in WebView/OS/native libraries are instantly erased.
- Recovery identity/private key is generated/exported during setup and confirmed by a decrypt-a-challenge round trip. Do not persist the recovery private key in normal app data or logs. A user-held private identity plus the encrypted backup is the only off-machine portable recovery path.
- Ordinary user password reset must never silently discard/re-key clinic data. Account recovery and database-key recovery are separate. A valid local DPAPI profile can still open the DB after app password reset; a new Windows profile uses the explicit backup recovery flow.
- For `K_db` rotation: acquire a maintenance lock, create/verify a portable backup, stage the new DPAPI-wrapped key, rekey via SQLCipher's supported rekey operation, run integrity/schema/attachment checks, then commit version metadata. On any interruption, startup must deterministically select the old verified state or complete/rollback without guessing. Prove this state machine before implementation. Attachment key rotation is a separate audited re-encryption operation.

## Why this design

- DPAPI user scope binds the local protected key to the same Windows user profile/credentials and machine context; it is stronger than an unencrypted key file but is intentionally **not portable**. Microsoft notes that machine-scope DPAPI allows any user on that machine to decrypt, so it is rejected for the local key.
- SQLCipher encrypts all SQLite database pages (including derived tables and the FTS index), not only selected columns, while keeping repository/transaction behavior SQLite-based.
- The recovery public-key approach supports scheduled backups without storing a plaintext backup passphrase. The app can encrypt with a public key while the owner keeps the private key offline. See ADR-0004 for format and restore details.
- A separate application login password and database key avoid coupling user accounts to one shared DB key and make role revocation possible without re-encrypting the database.

## Alternatives rejected or deferred

| Option | Reason |
|---|---|
| Plain SQLite plus filesystem ACL only | Does not protect a copied DB from a different profile or removable-media exposure; not acceptable for an encrypted-at-rest claim. |
| DPAPI LocalMachine scope | Any user on that PC can potentially decrypt; violates least privilege. |
| Use clinic admin login as SQLCipher password | Password rotation/recovery and shared users become coupled to a single database key; app password is not necessarily high entropy. |
| Store SQLCipher key in source, config JSON, registry or plaintext sidecar | Direct key disclosure. A filename obfuscation/hash is not key protection. |
| Official SQLCipher .NET Commercial/Enterprise package | Paid license conflicts with the no-paid-runtime/dependency constraint absent a later owner-approved change. |
| Legacy third-party .NET SQLCipher bundle | Microsoft documents it as unofficial; NuGet reports legacy/deprecated. Not a safe default without a current maintainer/build/license proof. |
| Custom encryption for the entire database implemented by the app | Easy to miss SQLite pages/WAL/temp files and create a nonstandard database format; SQLCipher is preferred if its build/backup gates pass. |

## Threat boundary and residual risks

- Protects encrypted DB/attachments and portable backups against casual file copying, lost removable media and unauthorized Windows profiles without the owner-held recovery identity.
- Does **not** protect an unlocked designated Windows profile from malware running as that user, a determined local administrator, kernel compromise, an instrumented/tampered application binary, a user who can invoke the app's privileged process, screenshots/exports, or a person who obtains the recovery private key.
- Application RBAC and re-authentication remain necessary for in-app staff roles. Recommend Windows account controls and BitLocker; do not claim FIPS certification, HIPAA/GDPR/Bangladesh-law certification or forensic tamper resistance.
- DPAPI output is machine/profile-bound. Restore to a new machine must decrypt a portable backup using the separately stored recovery identity and establish a new local DPAPI wrapper.
- Losing both the Windows profile-protected key and the recovery private identity can make all encrypted records/backups unrecoverable. This consequence must be displayed and accepted before setup finalization.

## Proof gates (all not run)

1. On Windows MSVC, build the exact pinned bundled SQLCipher + vendored OpenSSL feature set and inspect output/native notices.
2. Create DB, verify SQLCipher version, wrong-key failure, raw data-marker non-disclosure, secure reopen, foreign keys, WAL restart, migration rollback, read-only integrity check and real online backup/restore.
3. Run DPAPI `CurrentUser` protect/unprotect with the same user, reject corrupted blobs, verify another user profile cannot decrypt, and explicitly record machine/profile scope behavior.
4. Exercise two concurrent readers and serialized writes; test busy timeout, process crash, disk-full, key rotation and attachment encryption/recovery.
5. Measure connection-open/key KDF cost, DB growth, read/search, and backup under Phase 0 workloads on named Windows hardware. No performance score has been measured.
6. Review SQLCipher Community source version/signature, BSD-style license and required copyright/license/disclaimer plus OpenSSL and build-time dependency notices.

## References

- Microsoft DPAPI `CryptProtectData` and CurrentUser/LocalMachine scope: <https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata>
- Microsoft SQLite custom versions and bundle table: <https://learn.microsoft.com/en-us/dotnet/standard/data/sqlite/custom-versions>
- SQLCipher Community licensing/notice requirements: <https://www.zetetic.net/sqlcipher/community/>
- SQLCipher API/key/rekey: <https://www.zetetic.net/sqlcipher/sqlcipher-api/>
- Rusqlite features and bundled source/licenses: <https://docs.rs/crate/rusqlite/latest>
- SQLCipher online backup API discussion (must verify against selected version): <https://discuss.zetetic.net/t/using-the-sqlite-online-backup-api/2631>
- Phase 0 threats and recovery requirements: [`../../phase-0/risk-register.md`](../../phase-0/risk-register.md), [`../../phase-0/requirements-spec.md`](../../phase-0/requirements-spec.md)
