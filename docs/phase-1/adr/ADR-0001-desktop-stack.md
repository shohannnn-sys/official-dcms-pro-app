# ADR-0001 — Desktop stack and trusted application boundary

- **Status:** Provisional; architecture lock is blocked by Windows print, offline-runtime, SQLCipher/DPAPI and recovery proofs.
- **Date:** 2026-10-02 (UTC)
- **Decision owner:** Product owner
- **Scope:** Windows-only desktop client for one clinic on one local Windows user profile.
- **Supersedes:** Phase 0 recommendation in `docs/phase-0/architecture-candidates.md` only after the gates in this ADR are met.

## Context

The product must work without internet, preserve clinical and financial history, enforce authorization outside the UI, install on a non-developer Windows PC, print/PDF Bengali-capable clinical and financial documents, and avoid recurring/paid runtime services. Product scope is one local clinic database per PC; there is no live LAN sharing or cloud sync. Windows 11 x64 is the support target. No production application, production schema or migration exists yet; the isolated Phase 1 proof harness is explicitly separate.

Phase 0 compared Tauri, Electron, and native .NET. Scores were discovery judgements, not tests. Windows printing, portable encrypted recovery, and offline installation dominate the decision. Tauri is not selected because it is trendy or because it compiles on Linux: it is preferred only if its narrow Windows WebView2 adapter and key-recovery design pass the evidence gates.

## Proposed decision

Use **Tauri 2 + React + TypeScript + Vite + Rust + local SQLite with SQLCipher Community Edition** as the candidate stack, with the following boundary:

```text
React / TypeScript WebView
  └─ narrow, typed use-case IPC (no SQL, no Node, no general filesystem/shell)
       └─ Rust command boundary: validate DTO; derive actor from server session
            └─ Rust application/domain services: RBAC, business invariants, audit,
                 transaction/idempotency and safe error projection
                  ├─ repositories + versioned SQL migrations + SQLCipher
                  ├─ DPAPI/key, encrypted attachment and backup services
                  └─ Windows adapters: picker, printer, WebView2, scheduler
```

The frontend is a presentation layer, not a source of truth. Rust services authorize every read and write, including search counts/previews, reports, notifications and document rendering. No browser renderer, frontend plugin or generic CRUD command receives database access.

### Operating envelope

- Windows 11 x64 on a Microsoft-supported servicing release; Windows ARM and unsupported Windows versions are out of scope until separately agreed and tested.
- One active local clinic database under the designated Windows user's `%LOCALAPPDATA%`; the process uses one serialized writer and a bounded read pool if benchmarks justify it. Never locate a live SQLite/WAL database on a network share, OneDrive-synced folder or removable drive.
- Several in-app staff accounts can use the installation, but the baseline assumes a single designated Windows profile. Different Windows profiles do not share the clinic database. Owner confirmation is required if that is not acceptable.
- Local bundled UI, fonts, templates, database, backup and printer-driver integrations; no mandatory network, cloud, telemetry, analytics, external font, ad SDK, updater service or paid API. A bundled **offline Evergreen WebView2 installer** is the current packaging candidate. WebView2 itself may be serviced by Windows/Microsoft mechanisms when a network is available; app workflows remain usable disconnected.
- NSIS `.exe` is the candidate installer. Prefer a per-user install aligned with per-user data/DPAPI, but prove WebView2 provisioning/elevation and install/uninstall behavior on a clean Windows machine.

## Evidence and rationale

- Tauri's capability model can constrain which bundled windows can invoke app commands; custom Rust command handlers remain the trusted code that must enforce permissions. The Tauri security documentation explicitly treats a compromised/insecure Rust core, overly broad capability configuration and system-WebView vulnerabilities as trust-boundary risks.
- `tauri::webview::WebviewWindow::with_webview` exposes platform handles on the main thread, and Tauri advises pinning at least a minor version when using it because WebView2-related dependencies may change. This makes a small Rust/COM adapter plausible, not proven.
- Microsoft documents WebView2 APIs for system/browser print dialogs, selected-printer printing, custom page settings and PDF-to-stream. These are necessary ingredients for one HTML document engine; only a Windows run can validate their composition inside the selected Tauri version.
- Rusqlite documents a bundled SQLCipher build with vendored OpenSSL and a bundled-source commercial-use path under SQLCipher Community licensing/attribution. The exact pinned native build, license notices, Windows compilation, DPAPI and restore have not been tested.
- Electron remains the strongest fallback when the print API/renderer is the deciding issue. Its official `webContents.print`, `getPrintersAsync` and `printToPDF` APIs are more direct, but the extra bundled Node/Chromium runtime and renderer/preload hardening burden are material.
- Native .NET remains the strongest alternative for Windows-native UI/printing. The official SQLCipher .NET integration is commercially licensed; Microsoft's SQLite provider lists an unofficial legacy `bundle_e_sqlcipher`. Do not introduce a paid dependency or quietly choose an unmaintained native bundle. A native .NET switch would need a new data-encryption decision as well as print evidence.

## Alternatives considered

| Option | Why not the current proposal / when to revisit |
|---|---|
| Electron + React + TypeScript + main-process services | Revisit if the Tauri/WebView2 adapter cannot pass the print/PDF/thermal proof without brittle unsafe COM code, or if WebView2 variation makes output unacceptable. Larger runtime and strict `contextIsolation`, sandbox, CSP and preload allowlist are required. A native SQLite/SQLCipher binding must be rebuilt and audited for the exact Electron ABI. |
| Native WPF / WinUI 3 + C# | Revisit if Tauri printing/runtime/security gates fail and Windows-native behavior outweighs the React document/UI system. WPF/WinUI still need a proven Bengali document/PDF engine. The available SQLCipher .NET integration/licensing findings do not presently give a reviewed no-cost implementation. |
| Tauri + plaintext SQLite | Rejected if the app advertises encrypted-at-rest storage or if clinical/financial protection is a release requirement. SQLCipher failure must stop the gate; there is no silent plaintext fallback. |
| Browser app + local server | Rejected: adds a listening local service and broader origin/CSRF/port boundary for no required benefit. |

## Security and design consequences

### Required

1. Pin Tauri to a tested minor/patch version in the lockfile; do not upgrade `tauri`, `wry`/WebView2 bindings or the system WebView without running the native print/installer regression.
2. Permit only local bundled assets and an explicit development origin in development. Production CSP forbids remote scripts, remote fonts, remote images and arbitrary navigation. No Node integration, filesystem plugin, shell plugin or generic SQL API is exposed to the renderer.
3. Explicitly enumerate app commands in the Tauri build manifest and grant the main window the smallest capability set; use a second, more restricted capability for the print window. Services still authorize every call.
4. Keep all money arithmetic in integer poisha, all final clinical/financial snapshots immutable, and all consequential mutations transactional and auditable.
5. Enforce a single process/maintenance lock around restore, migrations and backup-sensitive operations. Use local files only for the live database and encrypted attachments.
6. Use offline WebView2 provisioning unless clean-machine testing demonstrates that the supported Windows 11 image always has a usable runtime. Do not rely on an online bootstrapper. Prefer Evergreen offline installer over a frozen fixed runtime unless proof shows otherwise; Evergreen must still be exercised disconnected at install and launch.
7. Have a reproducible Windows CI build and dependency/notice audit. No code signature or publisher identity is claimed until the owner supplies an authorized signing identity through a protected CI secret channel.

### Costs accepted only if gates pass

- Two languages and a typed IPC contract increase maintenance work.
- `with_webview`/COM interop is Windows-specific unsafe code; isolate it, keep it short, pin versions and test status/error mapping.
- The offline WebView2 package adds roughly 127 MB to the Tauri installer per current Tauri documentation. Measure the actual signed/unsigned artifact; do not describe documentation estimates as a measured size.
- Evergreen WebView2 reduces product responsibility for bundling engine security updates but is an OS/runtime dependency. Fixed runtime is not the default because the product would own a much larger artifact and every runtime security update.
- SQLCipher Community requires redistributing its required copyright/license text and applicable dependency notices.

## Validation gates before changing status to Accepted

| Gate | Required evidence | Current status |
|---|---|---|
| Windows native print/PDF | Tauri Windows app exercises the selected `ICoreWebView2_16` APIs; A4, A5 and supported 58/80 mm driver profiles; printer listing/selection; PDF artifact; cancellation/unavailable/error handling; long Bengali/English content. Include one representative physical clinic printer before release if hardware is available. | Not run. |
| Offline installer/runtime | Actual NSIS installer installs and launches on a reset supported Windows 11 x64 image with network disconnected and no development tools; verifies runtime behavior, upgrade/uninstall and clinic-data preservation. | Not run. |
| SQLCipher + DPAPI | Exact locked crate/native versions compile on Windows MSVC; fresh DB, wrong-key denial, unreadable plaintext marker, migration, WAL/restart, online backup and DPAPI user-scope round trip pass. | Not run. |
| Recovery | Encrypted backup with portable recovery identity restores to a different clean Windows profile; tampered/wrong-key/partial restore leaves active data intact. | Not run. |
| Owner scope | Confirm no live multi-PC sharing and the designated Windows-profile model are acceptable. | Phase 0 assumption; owner confirmation pending. |
| Dependency/notice | Lockfile, SBOM/license scan, binary/runtime inventory and shipped notices reviewed. | Not run; no manifests exist. |

A successful Linux compile, a WebView2 documentation page, or a generated PDF from a different host does not pass these gates. If any blocking gate fails, reopen this ADR and compare Electron/.NET with the actual defect and evidence. Do not mark the candidate architecture locked while a blocking gate is unresolved.

## References

- Phase 0 comparison and proof plan: [`../../phase-0/architecture-candidates.md`](../../phase-0/architecture-candidates.md)
- Tauri capabilities/security: <https://v2.tauri.app/security/capabilities/>
- Tauri `with_webview` API and minor-version note: <https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html>
- Tauri Windows offline WebView2 installer modes: <https://v2.tauri.app/distribute/windows-installer/>
- Microsoft WebView2 print APIs: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/print>
- Electron printing: <https://www.electronjs.org/docs/latest/api/web-contents>
- Microsoft SQLite custom native bundles: <https://learn.microsoft.com/en-us/dotnet/standard/data/sqlite/custom-versions>
- SQLCipher .NET licensing/integration: <https://www.zetetic.net/sqlcipher/sqlcipher-for-dotnet/>
