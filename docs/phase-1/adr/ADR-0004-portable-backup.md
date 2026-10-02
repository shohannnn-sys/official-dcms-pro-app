# ADR-0004 — Portable encrypted backup and recovery

- **Status:** Provisional; requires Windows SQLCipher online-backup, `age` stream, scheduled-task and cross-profile restore proof plus owner acceptance of the recovery-key UX.
- **Date:** 2026-10-02 (UTC)
- **Decision owner:** Product owner
- **Related decisions:** ADR-0002 (database key), ADR-0001 (deployment)

## Context

A backup must be a consistent, encrypted, self-contained point-in-time set that includes the encrypted database and managed attachments, works offline, supports unattended scheduled creation, and can be restored on a replacement Windows profile without a cloud KMS. It must not silently replace a healthy clinic database on a corrupt, wrong-key or partial restore.

DPAPI CurrentUser protection is intentionally machine/profile-bound, so a DPAPI-only backup cannot be portable. Storing a plaintext recovery password or private key beside the backup would defeat portability security. A scheduled job cannot require the owner to retype a secret at each run.

## Proposed decision

Use a versioned `.dcmsbackup` package protected by the open-source Rust [`age`](https://crates.io/crates/age) library's X25519 public-key streaming encryption, subject to exact version/license/security review.

### Recovery identity

- During clinic setup, generate a public/private X25519 recovery identity using the `age` format.
- Keep the public recipient identity in encrypted clinic settings/backup metadata; a public key is not secret and enables unattended scheduled encryption.
- Show/export the private recovery identity exactly once. The owner chooses an offline destination and confirms it by reopening the saved identity and decrypting a one-time challenge. Never persist the private identity in normal app data, diagnostics, logs, clipboard history or the backup destination.
- The owner must store the private identity separately from backup media. Explain that losing it makes off-profile encrypted backups unrecoverable. Require explicit confirmation before clinic setup is finalized.
- A future recovery-identity change retains old identities for old backups; the identity ID/public recipient is recorded per backup. Rotating the public recovery recipient does not automatically re-encrypt the database.

### Package contents and encryption

1. Put a versioned manifest, a SQLCipher-encrypted online database snapshot, already-encrypted opaque attachment objects, and a separately `age`-encrypted `K_db` wrap in a strict internal archive/stream.
2. Encrypt the complete archive stream with the clinic's current public X25519 recovery recipient. This hides metadata and gives ciphertext authentication/integrity. The key wrap is encrypted to the same recipient **again**, so a temporary archive created after outer decryption still contains no plaintext DB key.
3. Manifest fields are limited to format/schema/app/cipher versions, random clinic/database ID, recovery-recipient ID, creation time, bounded file IDs/sizes/hashes and record-count/integrity metadata where safe. No patient name, diagnosis, invoice number or phone goes in the unencrypted outer header or filename.
4. Archive entry names are generated opaque IDs and fixed metadata paths, never user-supplied paths. Restore rejects absolute paths, `..`, duplicate entries, symlinks/hardlinks, unexpected types, impossible sizes, excessive entry counts, invalid hashes, unsupported versions and unbounded decompression.
5. The format stays readable only by this app; do not depend on a paid utility or online recovery service. Exact `age` version, streaming API, archive container and compatibility policy are a proof/implementation choice. Changing encryption or archive formats later requires a migration/restore path, not a silent incompatible version bump.

### Create backup

- Acquire a backup snapshot boundary and serialize writes/maintenance. Use the SQLCipher Online Backup API from the exact selected build only if the Windows proof confirms encrypted-to-encrypted with the same cipher/key works; otherwise use the SQLCipher-documented export route selected by ADR update. Do not copy a live `.db`/WAL file by hand.
- Flush/checkpoint consistently; create an encrypted DB destination using the same key, verify it opens and passes `PRAGMA integrity_check`, and capture a stable set of encrypted attachment objects plus manifest hashes.
- Encrypt `K_db` to the public recovery recipient; pack only encrypted DB/attachments and the encrypted key wrap. Encrypt the complete package stream to the same recipient.
- Write to a restrictive same-volume `.partial` destination; flush file data, finalize the authenticated age stream, re-read/check file size/hash and atomically rename to a unique timestamped `.dcmsbackup`. Do not overwrite a prior valid backup silently. If cancellation/disk-full/locked-folder occurs, delete only the partial candidate and retain the last valid backup.
- Record an audit result and `backup_runs` status without secret, patient content or recovery identity. Surface failure on next app launch if the app was closed.

### Restore

1. Require `admin.backup.restore`, re-authentication, maintenance lock and explicit typed confirmation. Select exactly one `.dcmsbackup`.
2. Request the corresponding user-held private recovery identity. Stream-decrypt the outer age envelope to a restrictive same-volume restore staging area; the staged payload contains ciphertext-only DB/attachments and an inner encrypted key wrap. A wrong identity/authentication tag fails before active data is modified.
3. Validate the manifest, hashes, entry allowlist, size/version/schema compatibility and archive structure. Decrypt the inner `K_db` only into zeroizing process memory; open the staged DB with that key and run integrity, migration-compatibility, foreign-key, audit-chain and attachment-key/object checks.
4. Before switching, create and verify an encrypted pre-restore backup of the currently active clinic data. If this fails, stop and keep current data.
5. Stage the DB and attachment directory under the same local volume; write a recoverable restore journal; flush; close/revoke old sessions; atomically rename the active data directory to a retained previous directory and the validated stage to active. On startup, the journal deterministically recovers either the previously verified set or the new verified set. Never stitch a new DB to old attachments.
6. Protect `K_db` for the new Windows profile with DPAPI CurrentUser, reopen the activated DB, verify required invariants, write a restore audit event in the restored database, and only then report completion. Keep the previous directory until post-swap checks pass and retention permits cleanup.
7. Every injected failure before commit leaves the current DB usable; every failure during directory-swap recovery has a tested documented rollback. Cancellation before commit is no-change. After commit begins, finish or recover atomically rather than abandoning a half-swap.

### Scheduling

- A per-user Windows Scheduled Task may invoke a hidden, signed app maintenance mode while the designated user profile is available; do not store Windows account passwords in the task. It unwraps `K_db` with DPAPI and encrypts the snapshot using only the public recovery recipient.
- If the machine is off, the profile is unavailable or the destination is missing, mark the scheduled attempt missed/failed and run a catch-up attempt on the next app launch/logon. Do not claim backup success until the package is closed, flushed and verified.
- The task must use the same local single-instance/database maintenance lock and should not create a visible clinical UI. App shutdown, task registration, disabled task, permissions, changed drive letter, disk-full and missing destination are test cases.

## Alternatives rejected

- Plain ZIP/copy of database, WAL, attachments or recovery key: rejected because it leaks/risks inconsistent clinical data.
- DPAPI-encrypted backup alone: rejected because a new computer/profile cannot decrypt it.
- Password-encrypted archive with a password stored in the app to allow scheduling: rejected as unnecessary long-lived secret duplication. A public-key recipient allows unattended encrypt-only scheduling.
- Cloud backup/KMS or paid encryption service: rejected by offline/no recurring service constraints.
- Home-grown streaming AEAD file format: deferred; use the maintained `age` file format unless compatibility/manifest tests reveal a concrete gap.
- SQLite file copy after `BEGIN IMMEDIATE`: not the selected design because SQLCipher WAL, crash/locking and attachment consistency need a supported snapshot API or validated export.

## Consequences and owner-facing risk

- The owner must perform and protect one offline recovery-identity export. The UI must show clearly that the key is not recoverable by the clinic password or developer.
- X25519 recipient encryption supports unattended backups but does not help if both the active DPAPI profile and all private recovery identities are lost.
- The outer archive is confidential/integrity-protected, while inner ciphertext layers keep the DB key protected even in staging. OS ACLs are still required for temporary ciphertext and metadata.
- Large backups need streaming and bounded memory. Schedule/cancel/progress behavior requires measurement on realistic data and removable/network-selected destinations.
- Backups made under older format/cipher/schema versions must remain restorable for the declared support window; no compatibility horizon has been agreed yet.

## Proof gates (not run)

- SQLCipher Online Backup API/export on Windows, encrypted-to-encrypted same-key; source and destination integrity, WAL activity, process termination, disk-full and cancellation.
- `age` X25519 streaming round trip, wrong private key, corrupted/truncated envelope, large (>5 GB synthetic) input/output, memory profile and nonce/tag failures.
- DPAPI local open plus restore on a different Windows user profile using only the exported private identity; no plaintext database key in source, log, manifest, archive staging, process arguments or installer.
- Full staged restore fault-injection matrix, pre-restore failure, rename/crash journal recovery, old DB/attachment preservation and restart.
- Scheduler while app closed, current-user DPAPI, task disabled/offline destination, machine-off catch-up and accurate persisted failure notification.
- Owner review/acceptance of the recovery-key UX and historical-backup key rotation rules.

## References

- SQLite Online Backup API: <https://sqlite.org/backup.html>
- SQLCipher encrypted backup discussion (verify against exact build): <https://discuss.zetetic.net/t/using-the-sqlite-online-backup-api/2631>
- SQLCipher Community license/attribution: <https://www.zetetic.net/sqlcipher/community/>
- `age` format and Rust library: <https://age-encryption.org/v1>, <https://docs.rs/age/latest/age/>
- DPAPI user-vs-machine scope: <https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata>
- Phase 0 requirements/gaps/risk: [`../../phase-0/requirements-spec.md`](../../phase-0/requirements-spec.md), [`../../phase-0/gap-analysis.md`](../../phase-0/gap-analysis.md), [`../../phase-0/risk-register.md`](../../phase-0/risk-register.md)
