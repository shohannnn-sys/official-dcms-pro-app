# DCMS Pro — Phase 1 Architecture and Technical Design

**Status:** Design baseline drafted; **not architecture-locked**. Phase 1 started on 2026-10-02 (UTC) after the owner's `Continue`. Phase 0 remains the requirements baseline in [`../phase-0/`](../phase-0/).

Phase 1 produces design and proof plans only. No shipping application feature, production migration, production package, installer or production UI is being implemented here. A minimal isolated proof crate and Windows workflow exist only to test candidate SQLCipher, DPAPI and WebView2 API paths; they are non-shipping, synthetic and not the application test suite.

## Documents

### Architecture decisions (all provisional pending blocking Windows evidence)

- [ADR-0001 — Desktop stack and trusted boundary](adr/ADR-0001-desktop-stack.md)
- [ADR-0002 — Local encryption and key protection](adr/ADR-0002-data-encryption-and-keys.md)
- [ADR-0003 — WebView2 documents and Windows printing](adr/ADR-0003-webview2-printing.md)
- [ADR-0004 — Portable encrypted backup and recovery](adr/ADR-0004-portable-backup.md)

### Technical designs

- [System architecture, modules, trust boundaries, deployment model](system-architecture.md)
- [IPC and client-state contract](ipc-and-state-contract.md)
- [Relational ERD, data dictionary, constraints, indexes and transactions](data-model.md)
- [Authentication, RBAC, sessions, activation and audit security](security-and-rbac.md)
- [Backup, restore, scheduler and failure recovery](backup-and-recovery.md)
- [Document snapshots, HTML/CSS, Bengali fonts, preview, PDF and printing](documents-printing-fonts.md)
- [Windows packaging, CI, dependency/license policy and test strategy](windows-ci-dependencies-tests.md)
- [Proposed implementation folder structure](folder-structure.md)
- [Architecture review and unresolved proof gates](architecture-review.md)
- [Phase 1 evidence/status report](status-report.md)

## Decision discipline

The Phase 0 recommendation was Tauri 2 + React/TypeScript + Rust + SQLite, with candidate SQLCipher, DPAPI and WebView2 printing. This package makes that recommendation precise enough to test, but does not treat documentation or a successful compile as a Windows proof. Windows runtime/print behavior, SQLCipher/DPAPI recovery, and disconnected installation still need evidence. Until those gates pass, the stack and at-rest security plan stay **provisional**; no Phase 2 or production implementation is authorized.

No activation code, plaintext verifier, recovery private key, credential, or production secret belongs in these documents or in Git. See [`status-report.md`](status-report.md) for what has actually been run.
