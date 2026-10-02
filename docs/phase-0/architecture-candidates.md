# DCMS Pro — Architecture Candidate Evaluation (Phase 0)

**Status:** Evidence-based Phase 0 recommendation; the formal architecture is not locked. Phase 1 must prototype the critical Windows/SQLite/printing seams and record final decision(s) in ADR(s) before feature implementation.

## 1. Decision drivers

Ranked drivers from the owner requirements:

1. Windows 11 reliability, offline install/use and Windows printer/driver compatibility.
2. Strong service boundary so financial/clinical authorization and transactions cannot be bypassed through a UI route or IPC payload.
3. Reliable SQLite integrity/migrations/backups and no artificial patient-history cap.
4. Reliable A4/A5/thermal documents, direct PDF export, print preview, Bengali shaping and Windows printer selection.
5. Fast, polished responsive clinical UI on ordinary x64 clinic PCs, high DPI and small screens.
6. Self-contained commercial installer, no paid runtime dependency, no hidden network/telemetry, manageable dependency/license obligations.
7. Maintainability and repeatable Windows CI despite the current Linux-only agent host.

## 2. Candidates considered

### A. Tauri 2 + React + TypeScript + Rust + SQLite/SQLCipher — recommended baseline

**Shape:** React/TypeScript/Vite renders the user interface and local HTML/CSS clinical documents. Tauri 2 packages a native Windows shell and WebView2. Rust owns domain services, authorization, validation, database access, audit, backup/restore, file safety and narrow OS adapters. SQLite is accessed only behind repositories/application services; no frontend SQL plugin. A thin, audited Rust/Windows adapter uses documented WebView2/Windows print APIs for printer enumeration, selected-printer output and PDF. The database and selected binary data are encrypted at rest if the Phase 1 SQLCipher/DPAPI/recovery proof succeeds.

**Advantages:**

- Small native host and compiled Rust service layer; frontend has no Node access. Tauri v2 has explicit capability/permission configuration; commands can be narrow and checked in services rather than granting general shell/filesystem/database access.
- Rust makes the trusted business/data boundary explicit. Typed command DTOs and tests can make authorization, transaction and audit requirements independently testable.
- React/TypeScript supports a polished custom application UI, responsive grids, accessible component patterns and the same HTML/CSS primitives for patient-facing print layouts.
- SQLite in-process is a strong match for one offline PC. SQLCipher Community Edition is a candidate commercially distributable encryption layer with required attribution; must prove Windows/MSVC build, performance, migration and backup behavior before adopting.
- WebView2 has native print-dialog, printer-directed print and PDF APIs, making the Windows driver path possible without a paid PDF service. The API is documented by Microsoft. Rust COM integration is a deliberate spike, not an assumption.
- An offline WebView2 installer can be bundled by Tauri (the official Tauri documentation reports roughly 127 MB of additional installer content); Windows 11 also normally supplies WebView2. The package can therefore install/use locally without downloading a runtime during customer installation.
- All runtime UI, database, fonts and documents are local; no runtime server or internet is required.

**Costs and risks:**

- Two implementation languages and an IPC boundary require disciplined ownership, typed contracts and testing.
- Tauri does not provide a first-party complete printer-management/document-publishing subsystem. Native WebView2 COM APIs, installed printer selection, custom paper behavior, cancellation and printer errors must be prototyped and maintained in a small Windows-specific adapter.
- WebView2 is system-managed unless a fixed runtime is bundled. Offline installer, supported Windows baseline, minimum API version and servicing behavior must be proven on a clean PC. Embedding a fixed runtime may increase installer size and makes runtime security updates a product responsibility.
- SQLCipher adds native C/OpenSSL/MSVC build and license-notice work; key recovery and backup portability are critical. A stock unencrypted SQLite file would be an unacceptable silent fallback if the security design promises encrypted data.
- Current sandbox lacks Rust, Windows, WebView2, MSVC, browser automation and printers; Phase 1 proof and all Windows release evidence must use Windows CI/acceptance machines.

### B. Electron + React + TypeScript + SQLite — strongest fallback for renderer/printing ergonomics

**Shape:** Electron main process and narrow preload bridge host the same React UI; a local database adapter (normally a native SQLite binding) runs only in the trusted main process. Chromium renders document HTML; Electron `webContents.print` and `printToPDF` provide mature platform printing/PDF entry points.

**Advantages:**

- Bundles Chromium and Node, so no system WebView prerequisite and renderer behavior is more tightly controlled across clean machines/offline installs.
- Official Electron APIs cover printer-directed printing and PDF generation; excellent fit for repeatable HTML/CSS document templates and custom paper settings.
- One TypeScript/JavaScript ecosystem can reduce language/tooling changes; React UI, PDFs and browser automation have broad tooling support.
- Mature, well-understood Windows packaging options.

**Costs and risks:**

- Bundled Chromium/Node yields a larger install, higher baseline resource use and a larger security-sensitive runtime surface. Browser-engine security updates would have to arrive through a new product build; an absolute “never update” policy is unsafe.
- Electron's isolation guidance is strict: no renderer Node integration, context isolation and process sandboxing, restrictive CSP, local-only content and a minimal preload API. A mistake can expose native capabilities. Rust can still be used, but then adds another bridge without Tauri's natural command/capability model.
- SQLite usually requires a Node native addon; Electron ABI rebuild, Windows signing/packaging and SQLCipher native build coordination are additional failure points.
- It is a viable alternate if Phase 1 cannot deliver reliable Tauri/WebView2 print/PDF behavior or if embedded Chromium stability proves more important than package/runtime footprint.

### C. Native .NET WPF / Windows App SDK (WinUI 3) + SQLite — strongest direct Windows-native option

**Shape:** C# services and native Windows UI, with SQLite via a .NET provider; WPF can use its printing/XPS APIs. WebView2 may still be used for sophisticated HTML clinical documents and PDF to preserve the common HTML layout system.

**Advantages:**

- Direct access to Windows file/folder dialogs, spooler/printer settings, OS session/security, task scheduler and mature native printing. Microsoft documents rich WPF XPS print APIs.
- Native controls can offer familiar Windows keyboard/focus/accessibility and high-DPI behavior; one C# domain/service language can simplify database/business code.
- Self-contained .NET packaging and Windows-native diagnostics are well understood.

**Costs and risks:**

- Modern premium dashboard/calendar/table/document visual work is possible but requires a custom XAML design system and different skills/tooling; it is less aligned with the React design system and print-template ecosystem.
- WinUI 3 deployment/runtime/Windows App SDK choices need verification. WPF printing has strong Windows integration, but complex modern Bengali prescription layouts, browser-like line wrapping and PDF export may still need WebView2 or another engine.
- The local agent has no .NET SDK, MSBuild or Windows host, so the same CI/acceptance constraint remains.
- This candidate should replace the recommendation only if the Phase 1 print spike identifies a Tauri/WebView2 limitation that a maintained native adapter cannot safely satisfy.

## 3. Weighted decision snapshot

Scores are discovery judgements (1 low, 5 high), not benchmark results. “Printing” includes the engineering risk of the complete required document system, not merely whether an API exists.

| Driver | Weight | Tauri | Electron | Native .NET | Rationale |
|---|---:|---:|---:|---:|---|
| Windows print/PDF path | 5 | 4* | 5 | 5 | Electron APIs are direct; .NET is native; Tauri uses documented WebView2 APIs but needs a tested adapter. |
| Offline installation | 5 | 4 | 5 | 4 | Tauri can bundle offline WebView2; Electron bundles engine; .NET runtime/package configuration must be proven. |
| Security boundary / least privilege | 5 | 5 | 4 | 4 | Tauri Rust commands/capabilities align well; Electron is safe only with disciplined isolation; native .NET still needs strict service authorization. |
| Local DB integrity/encryption | 5 | 4* | 4* | 4 | All can host SQLite; SQLCipher/native package feasibility and recovery are a shared Phase 1 proof. |
| Premium responsive UI | 4 | 5 | 5 | 4 | React supports a precise reusable visual system; native XAML can be excellent but requires a separate implementation system. |
| Performance/resource footprint | 3 | 5 | 3 | 5 | Tauri native host with system WebView is expected to be light; Electron bundles Chromium; native UI is light. Must measure. |
| Build/package maturity for this scope | 4 | 4 | 5 | 4 | Electron is familiar for bundled web UI; Tauri Windows packaging is viable; .NET is strong on Windows. All require Windows CI. |
| Maintainability / module boundaries | 4 | 4 | 4 | 4 | Each can be layered; Rust/TypeScript split versus one stack is a trade-off, not a pass/fail. |
| License/commercial fit | 4 | 4 | 4 | 4 | All have permissive core options; every transitive/runtime/font/icon obligation still needs audit. |
| Indicative weighted total / 5 |  | **4.31*** | 4.38 | 4.21 | Near enough that the Phase 1 proof gates, not score arithmetic, decide final lock. |

`*` Dependent on Phase 1 validation. The apparent small score differences are not evidence. Electron is narrowly ahead on turnkey print/runtime; .NET on Windows-native behavior. Tauri is recommended because its least-privilege native service boundary and modest client footprint better match the combined security/offline product, provided the Windows print and SQLCipher gates pass.

## 4. Phase 0 recommendation (not yet the Phase 1 ADR)

Proceed to Phase 1 with **Tauri 2 + React + TypeScript + Vite + Rust + SQLite**, with a **candidate** SQLCipher Community Edition encryption layer, Windows DPAPI-protected local key material, and local HTML/CSS document templates rendered by WebView2. Keep the Windows shell adapter small and isolated. Use only Tauri's official dialog/plugin interfaces where adequate; do not delegate business rules, permissions or raw SQL to a third-party frontend plugin.

A recommended separation is:

```text
React UI / page features / design system
        │ typed DTOs over narrowly exposed Tauri commands
        ▼
Rust command boundary (session check, DTO validation, safe errors)
        ▼
Application use cases (permission checks, policies, transactions, audit)
        ├── Domain rules (money, clinical history, appointment, stock, RBAC)
        ├── Repositories (parameterized SQL, migrations, SQLite/SQLCipher)
        ├── Platform ports (DPAPI, dialog, file storage, notifications, lock)
        ├── Document service (persisted snapshots → local HTML/CSS profiles)
        │       └── Windows WebView2/print adapter → Windows spooler/PDF
        └── Backup/restore service (consistent snapshot, staged validate/swap)
```

The webview must have no Node API, no direct database, no remote URLs and no broad filesystem/shell command. Each Rust command accepts a validated use-case DTO, derives the user from the active session (never caller-supplied identity), applies server-side authorization/business rules, commits audit and data correctly, and returns a redacted structured result.

### Candidate frontend/backend dependency families (not pinned)

- UI/runtime: Tauri 2, React, TypeScript, Vite; accessible custom components or permissively licensed headless components; a small set of open-source icons and data-grid/chart tools only when required.
- State/forms: query cache for persisted IPC data, local-only store for transient UI, schema-validated forms. The Rust boundary repeats validation.
- Native/backend: stable Rust MSVC target, Tauri capability system, `rusqlite` or SQLx behind repositories (decide after SQLCipher/transaction/async spike), SQLCipher Community Edition only if technically/legal/recovery gates pass, Argon2id, Windows DPAPI, typed WebView2/Win32 interop.
- Database: local SQLite with versioned migrations, foreign keys, indexed query paths, WAL/safe durability settings benchmarked on Windows, one serialized writer and bounded readers; never open the active database over a network share.
- Documents: locally bundled Noto Sans and Noto Sans Bengali (SIL Open Font License; exact font files/license notices verified), CSS `@page` templates with independent paper profiles, WebView2 preview/print/PDF. Fonts do not load from the internet.
- File selection: native Windows picker, app-owned opaque file names, safe content validation and restrictive storage directories.
- Test tooling: Rust unit/integration suite and TypeScript UI unit/component suite; Windows WebView2 end-to-end/smoke suite; automated PDF text/geometry checks plus rendered-image review on Windows.

## 5. Phase 1 proof gates before ADR lock

1. **Print spike (blocking):** In a minimal Tauri Windows test host, use WebView2 to list Windows-installed printers, select one, invoke system print preview/dialog, print to Microsoft Print to PDF/direct WebView2 PDF, configure A4/A5 and at least one 58/80 mm profile, capture cancellation/offline/error states, and inspect Bengali, long text, pagination and signature area. Test with Windows 11 and one representative installed physical printer if available. Confirm the exact Rust COM API/versions are supported and stable enough to own.
2. **Offline installer/runtime spike (blocking):** Build the candidate Windows NSIS installer with offline WebView2 provisioning; install on a fresh supported Windows VM with network disconnected and no development tools. Verify runtime permissions, launch, uninstall and user data retention. Compare a bundled offline Evergreen installer with fixed-runtime size/update costs.
3. **SQLCipher/DPAPI spike (blocking for encrypted-at-rest claim):** Compile on `windows-latest` x64 MSVC; prove clean open, wrong-key denial, migration, read-only integrity checks, simultaneous readers/serialized writes, online backup, restore and recovery on same/new Windows profile. Review SQLCipher Community terms and all native/transitive notices. Do not quietly downgrade to plaintext SQLite if this fails.
4. **Backup threat/recovery spike (blocking):** Decide the owner-operable recovery key/passphrase flow; exercise scheduled and manual encrypted backup, attachment inclusion, integrity rejection, pre-restore backup, staged atomic restore, disk-full/locked-folder failures and cross-machine restore. Explain unrecoverability if a required recovery secret is lost.
5. **Database/scale spike:** Build a first normalized ERD, benchmark realistic test volumes on a named Windows baseline, verify DB remains on local storage, and establish search/profile/backup latency thresholds before schema lock.
6. **Security/build spike:** Confirm a narrowly scoped Tauri capability/command design, Windows installer signing plan availability, SBOM/license scanner output and GitHub Windows build commands. No unsigned/untested installer can be called release ready.

### Phase 1 fallback decision rule

If print/PDF/thermal paper or encrypted SQLite is not reliable after reasonable Windows-native investigation, pause Phase 1 and compare Electron and native .NET again with the measured defect and test artifact. Change stack only through an ADR explaining the evidence and migration implications. Do not replace a required printer or security feature with a fake approximation.

## 6. Application module plan

| Layer/module | Responsibility and forbidden shortcut |
|---|---|
| Presentation | Screen composition, accessibility, form field feedback, loading/empty/error states. It does not own permissions, balances, audit decisions or SQL. |
| UI state/contracts | Typed IPC contracts, query cache, transient dialog/navigation state. Caller cannot supply the trusted actor identity or permission result. |
| Application services | Use cases for setup/auth, patients, clinical, appointments/queue, documents, billing, payments, stock, accounting, search, notifications, staff/admin, audit, backup/restore. Enforce validation and RBAC here. |
| Domain | Money/poisha, invoice/payment allocation, permissions, clinical records/history, appointment and stock invariants. Independent unit tests. |
| Persistence | Versioned migrations, repositories, parameterized SQL, constraints, transactions, database key opening, backup API and integrity checks. No arbitrary UI SQL. |
| Security | Argon2id credentials/activation verifier, sessions/lock, permissions, DPAPI/key handling, path policy, audit integrity. Least privilege and explicit threat boundary. |
| Documents and printing | Persisted document snapshots → local template → profile preview → PDF/Windows printer; Bengali-safe fonts/wrapping/pagination. No paid conversion API or default A4 scaling. |
| Backup/restore | Consistent protected package, user-selected folder, validation/staging/pre-restore backup/atomic swap, progress and recovery. |
| Platform integration | Windows install/runtime, file/folder/save dialogs, WebView2 printing, DPAPI, scheduler, notification/shortcut, application data directory and error mapping. |

## 7. Proposed project structure (confirm in Phase 1)

```text
/
├── ARENA.md
├── docs/{phase-0,architecture,security,database,printing,backup,testing,release}/
├── .github/workflows/{ci,windows-release}.yml
├── frontend/
│   ├── src/app/                 # shell, route registry, auth/permission guards (UX only)
│   ├── src/features/{dashboard,patients,clinical,appointments,queue,prescriptions,billing,inventory,accounting,staff,settings,backup,search,notifications}/
│   ├── src/components/          # accessible design-system primitives
│   ├── src/contracts/           # generated/validated IPC DTOs; no database SQL
│   ├── src/documents/            # local templates and print styles
│   └── public/fonts/             # licensed local font files + notices
├── src-tauri/
│   ├── src/commands/             # narrow typed command entry points
│   ├── src/application/          # use cases, session/permission/audit orchestration
│   ├── src/domain/               # policies and calculations
│   ├── src/persistence/           # SQLite repositories and migration runner
│   ├── src/security/              # password, activation, DPAPI/key and file policy
│   ├── src/documents/             # immutable render models / print coordination
│   ├── src/platform/windows/      # Windows-only printer/scheduler/system adapters
│   └── migrations/                # versioned SQL, immutable once shipped
├── tests/{fixtures,integration,e2e,performance,documents}/
├── licenses/ and THIRD_PARTY_NOTICES.md
└── art/{icon-source,brand-source}/
```

Do not create every directory mechanically before Phase 1 architecture review; the purpose is clear ownership, not folder-count compliance.

## 8. Reference material reviewed

- Tauri Windows installer/runtime options: <https://v2.tauri.app/distribute/windows-installer/>
- Tauri security/capabilities: <https://v2.tauri.app/security/capabilities/>
- Microsoft WebView2 print, print-to-PDF and print-settings paths: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/print>
- Electron `webContents.print` / `printToPDF`: <https://www.electronjs.org/docs/latest/api/web-contents>
- Electron security recommendations: <https://www.electronjs.org/docs/latest/tutorial/security>
- WPF printing overview: <https://learn.microsoft.com/en-us/dotnet/desktop/wpf/documents/printing-overview>
- SQLite implementation limits: <https://sqlite.org/limits.html>
- SQLite appropriate-use/concurrency guidance: <https://www.sqlite.org/whentouse.html>
- SQLite WAL documentation (including network-filesystem limitations): <https://www.sqlite.org/wal.html>
- SQLCipher license information: <https://www.zetetic.net/sqlcipher/license/>
- Argon2id standard: <https://www.rfc-editor.org/rfc/rfc9106.html>
- Microsoft Windows 10 support lifecycle: <https://www.microsoft.com/en-us/windows/end-of-support>
