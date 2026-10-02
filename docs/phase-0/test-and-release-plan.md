# DCMS Pro — Testing, Acceptance and Release Plan

**Status:** Planned at Phase 0. No application tests/builds have been run. Every future “passed” claim must cite the exact command/environment/count and artifact/commit.

## 1. Test levels and evidence

| Level | What it proves | Planned evidence / execution environment |
|---|---|---|
| Static / source analysis | Types, lint, unsafe patterns, dependency health and production-only configuration | Frontend lint/type checks; Rust format/clippy/static checks; secret/network/dead-code scans; `cargo audit`/license/SBOM tooling on locked dependencies. Windows-specific build warnings treated as failures where practical. |
| Unit | Deterministic domain policy and edge cases without UI | Rust tests for money/poisha, permission predicates, activation verifier, session timeout, appointment transitions, chart numbering/surface normalization, prescription models, invoice/payment/reversal allocation, stock math, accounting aggregation, search redaction, audit canonicalization and backup manifest validation; TS tests for components/state/validation/formatting. |
| Database integration | Real constraints, persistence and transaction outcomes | Temporary SQLite/SQLCipher files; clean DB, ordered migration, migration failure rollback, FK/check/unique/index tests, query/result authorization, concurrent writes, WAL/restart, consistent backup, corrupted-file/wrong-key and restore. Run against the exact Windows native DB build. |
| Application-service integration | Business rules remain enforced outside UI | Invoke Rust use cases with real services and sessions. Forge object IDs, roles, route payloads, filters, export/search parameters; assert denied results and no DB change. Test audit/data transaction atomicity and safe error mapping. |
| Component/UI | Visible behavior, keyboard, validation and data states | Render actual screens/primitives, interaction/accessibility checks, filters/tabs/modal open/close/save/cancel, long text, Bengali and reduced-motion/focus. No production fake seed data. |
| Desktop E2E | Real Tauri/Windows/WebView2 bridge and key workflows | Choose and prove the compatible WebDriver/automation route in Phase 1. Windows runner launches packaged app; UI actions plus real DB assertions. Native dialogs/printer APIs get separate Windows adapter/service tests where UI automation is unreliable. |
| Document/PDF | Actual layout, text, paper geometry, pagination and rendering | Generate real prescription/invoice PDFs from persisted snapshots using WebView2. Inspect PDF metadata/page dimensions/extracted text/font embedding; rasterize with permissively licensed local tooling for visual review at each paper profile. Bengali/long text, multi-page invoices, signature space, margins and totals. Retain only intentional test artifacts. |
| Windows platform | Installer, OS, WebView2, DPI, spooler and local files | Windows 11 x64 CI and isolated clean Windows VM/machine. Disconnected install, first launch, native pickers/DPAPI/scheduler, Windows printer enumeration, Microsoft Print to PDF, representative installed physical printer, scaling/resolution, restart, uninstall/data retention. |
| Stress/performance | Large relational histories remain usable on ordinary hardware | Deterministic synthetic data generator available only to test builds; documented Windows 11 x64 reference machine. Benchmark cold/warm start, list/search, profile/timeline, backup/restore, PDF and memory at volumes below. Do not ship generated data or generator UI. |
| Release acceptance | Exact production artifact can be installed and used by non-developer | Isolated supported Windows install; activation/setup; representative clinic workflow; PDF/print; backup/restart/restore; uninstall/data policy; artifact signature/metadata/checksum; exact tag/commit/artifact hash recorded. |

## 2. Provisional performance workload and targets

Use a named Windows 11 x64 reference PC (initial target: 4-core CPU, 8 GB RAM, SSD, 1366×768 at 100% scaling) plus a lower-spec supported machine if available. Phase 1 must finalize reference hardware and baseline before performance implementation is tuned. Test data are synthetic and contain no real patient names or contact details.

**Initial stress dataset:** 100,000 patients; 500,000 visits; 1,000,000 timeline/status events; 250,000 prescriptions with multiple medicine lines; 250,000 invoices; 500,000 payment/allocation/reversal rows; 100,000 appointments; 100,000 inventory stock transactions; 100,000 audit events; 50,000 attachment metadata rows and a separately measured representative attachment corpus (up to 5 GB in a stress run).

**Initial measurable targets (re-baseline only with written Phase 1 rationale):**

- Cold start to usable login/shell: p95 ≤ 5 seconds; warm reopen ≤ 3 seconds.
- Patient code/name/phone search p95 ≤ 500 ms at 100,000 patients; debounce and cancel stale requests.
- First page of a normal indexed patient/invoice/inventory list p95 ≤ 800 ms; no unbounded result transfer to UI.
- Patient profile summary and most recent timeline slice p95 ≤ 1 second at the stated history volume; older timeline pages load incrementally.
- A typical one/two-page prescription PDF p95 ≤ 3 seconds; no UI freeze.
- Backup/restore of small and large datasets: measure duration and throughput; visible progress, bounded memory, successful cancellation/failure cleanup and no loss are hard requirements. Phase 1 sets operational duration budgets after measuring Windows disk/crypto performance.
- Track peak app memory, DB growth, attachment storage, PDF output and report time. A fast result that bypasses authorization or integrity is a failure.

## 3. Acceptance coverage requirements

`docs/phase-0/acceptance-matrix.csv` is the machine-readable planned matrix; all records are initially `Not Run`. The human-readable acceptance oracle is `requirements-spec.md` plus `permission-matrix.md`. Each accepted case must ultimately have: test ID, mapped requirement ID, positive flow, negative/error flow, role/permission case when sensitive, cancel/safeguard case when destructive, expected persisted state, environment, evidence path/hash, actual result and defect link.

Required end-to-end journeys include:

1. Clean installation → activation success/failure → interrupted wizard recovery → successful clinic/dentist/owner setup → close/reopen → login/wrong password/lock/unlock.
2. New patient → duplicate patient code rejection → profile → multiple visits/chart history → appointment → arrival/queue → completed/no-show/reschedule history.
3. Multi-medicine prescription draft/edit/finalize → A4/A5/narrow preview → Bengali PDF export → native printer choice → printer unavailable/cancel path.
4. Treatment catalog change after invoice → old invoice snapshot unchanged → partial payment → another allocation/full payment → due reconciliation → refund/reversal → permission-restricted reports/search/dashboard.
5. Inventory opening/stock-in/consumption/adjustment → negative-stock prevention → low-stock/expiry alert → supplier/purchase expense reconciliation.
6. Staff account with custom role → permitted and forbidden routes/IPC/search/projections → role revocation/session refresh → audit review.
7. Attachment picker/type/size validation/preview/missing file/delete permission → backup includes it → restore returns it.
8. Manual/scheduled backup → checksum/schema/wrong-key/corruption failures → pre-restore backup → restore success → injected restore failure preserves active data → restart verification.
9. Global search and notification read/unread across modules, with role-specific results and no restricted preview leakage.
10. Installer, icon/version metadata, disconnected clean-machine operation, PDF/print, uninstaller and user-data retention.

Every destructive operation needs a negative “Cancel makes no change” case. Every privilege-sensitive operation needs at least one directly invoked unauthorized service/IPC case in addition to UI visibility tests. Every document layout must be tested on A4, A5 and each implemented narrow/custom profile; every real app data screen must exercise populated, empty and error states.

## 4. UI, accessibility and visual verification

- Automate CSS/viewport checks for 1024×768, 1280×720, 1366×768, 1440×900, 1920×1080, 2560×1440 and 3840×2160. Use real Windows display scaling at 100%, 125%, 150%, 175% and 200% where a Windows VM/device permits; a browser viewport simulation is not proof of Windows DPI behavior.
- Audit dashboard, list/detail forms, calendar, queue, dental chart, prescription editor/preview, invoice, payment/accounting, inventory, user/role, backup/restore, settings, global search and dialogs.
- Check keyboard-only paths, visible focus, screen-reader semantics where tools permit, high contrast/reduced motion, field-level errors, hover/active/disabled/loading/empty/error states, scrolling, overflow, icon optical alignment, and Bengali strings in controls/content.
- Keep screenshots and PDFs as test evidence outside tracked production directories or in intentional test-artifact storage; do not ship them or test data.
- Phase 2 and Phase 11 visual audits require human review of actual Windows screenshots. A generated screenshot or compilation alone is not a visual pass.

## 5. Security, backup and failure-injection tests

- Wrong password, duplicate username, disabled/locked account, brute-force throttling behavior and no credential logging.
- Wrong activation input, successful activation, persisted one-time state, restart/reinstall/data-preservation behavior, verifier not found as plaintext, no development bypass in production.
- Every permission family: query/use-case direct denial, cross-patient ID forgery, unauthorized counts/aggregates/search snippets/export/notifications, stale frontend state and active-role revocation.
- SQL injection-shaped names/filters, malformed JSON/IPC DTO, path traversal/reparse-point and file type/content mismatch, over-limit file, corrupted image/PDF and external-open failure.
- Duplicate code/invoice, FK deletion restrictions, transaction rollback at each multi-write boundary, simultaneous queue/payment/stock action, integer rounding and reversal invariants.
- DB wrong key/corruption/locked/read-only/permission denied/disk full; migration interrupted; backup missing attachment; backup archive traversal/corruption/schema-too-new; wrong recovery secret; pre-backup failure; atomic restore swap failure; cancellation cleanup.
- Windows printer missing/offline/busy, unsupported size, user cancels dialog, Windows Print to PDF cancelled, no disk space, inaccessible export destination, print API exception; no false success.
- Restart after every critical create/edit/finalize/backup/restore action; verify DB/file state rather than toast only.
- Runtime network audit: run an installed app with network blocked and inspect outbound connections/host logs; source scan for HTTP client use, external asset URLs, analytics/crash SDKs/updaters. No required product flow may call a remote service.

## 6. GitHub Actions and production release workflow

### Pull-request/branch CI

- Trigger on PR and pushes to the Arena working branch; use least-privilege GitHub token permissions, pinned actions/versions and dependency lockfiles.
- Linux may run frontend lint/type/unit and pure Rust tests if the Rust toolchain is available; Windows `windows-latest` is authoritative for MSVC/Tauri/WebView2-native compile, database encryption and packaged tests.
- Windows job: install pinned Node/Rust toolchains and WebView2/VS prerequisites available to the runner; `npm ci`; frontend lint/type/test/build; `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; Rust unit/DB/integration tests; Tauri Windows production build; installed-app smoke/E2E; document/PDF checks; bundle/secret/license validation.
- All required jobs are branch-protection/release gates. Do not use `continue-on-error`, `|| true`, skipped mandatory suites, or release steps that publish after an upstream failure.
- Retain build/test artifacts on success and failure as appropriate; keep `node_modules`, Rust target, caches, temporary PDFs, logs and synthetic DBs out of Git/artifact release bundles.

### Production artifact workflow

- Build only on a clean Windows runner from a versioned tag/verified commit, with a production release profile. Use one agreed x64 installer format (candidate NSIS `.exe`; revisit in Phase 1) and only intentionally supported optional portable artifacts.
- Validate product/version/publisher metadata, custom `.ico` and source notice, WebView2 offline/runtime mode, install/uninstall registration, release payload, no dev server/source maps/devtools/test users/bypass/mock seed/secret/console spam.
- Generate SHA-256 checksums; include release notes, product version, installer, relevant notices and the tested commit/hash. Sign and timestamp only if owner supplies an authorized certificate via protected GitHub secrets; never request/store secrets in chat.
- Release job depends on all required CI/release-acceptance jobs and a manually approved GitHub environment/tag process. Use GitHub Release only if repository permissions permit and the artifact is real. If a verified permission/platform issue prevents it, retain only intentional final artifacts/support files in repository `dist/` and report the limitation; do not claim publication.
- Keep source state, installed artifact hash, test run, checksum and release entry linked so the artifact tested is exactly the artifact published.

## 7. Windows clean-machine and printer validation

The local Agent sandbox cannot perform this test. At Phase 13, use a reset/isolated supported Windows 11 x64 VM or physical machine without the project/source tree, Node, Rust, Python, Git, SDKs or developer-specific dependencies. Verify disconnected install/runtime, first activation/setup, user and dentist, patient, visit/chart, appointment/queue, prescription, invoice/payment, stock and expense, backup, close/restart, controlled restore, A4/A5/narrow PDF output, Windows printer selection (including Microsoft Print to PDF if available), physical Windows-installed printer if available, and uninstall/data behavior. Capture build version/hash, OS build, resolution/scaling, printer/driver, tests and outcome. If isolation, runtime, physical printer or required test access is unavailable, report it as not run and keep the relevant release criterion blocked.

## 8. Phase 0 test-state statement

Tests run during Phase 0 were repository/Git/file inventory, README read, toolchain/host inspection and documentation consistency checks as described in the phase report. No app exists yet, so no app unit, integration, migration, E2E, print, Windows install or clean-machine test has run. The acceptance matrix is a plan, not test evidence.
