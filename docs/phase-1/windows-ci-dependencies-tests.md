# Windows delivery, CI, dependencies, licenses and test strategy

**Status:** Proposed production pipeline and audit policy. There is no production package manifest, product CI, application/release binary, release artifact or product SBOM. The only workflow is the isolated, non-shipping Phase 1 proof job under `.github/workflows/phase-1-windows-proof.yml`; it is not application CI.

## 1. Supported build and deployment model

### Target and installer

- **Runtime target:** Windows 11 x64 on a Microsoft-supported servicing release. Windows 10 and Windows ARM are excluded unless the owner explicitly revises scope and supplies evidence.
- **Build target:** `x86_64-pc-windows-msvc`, built on a Windows hosted runner; no Linux cross-compilation is the authoritative release path.
- **Installer candidate:** Tauri NSIS `.exe`, per-user if WebView2 installation and OS integration pass. Start Menu entry and optional desktop shortcut are configured; product/version/icon and uninstaller data policy are tested.
- **WebView2 candidate:** Tauri `offlineInstaller` mode embedding the Evergreen offline runtime installer (Tauri documents an approximate additional 127 MB). This is preferred over the online bootstrapper so install does not require internet. Fixed runtime (~180 MB per Tauri docs) is not the default because the product would have to ship every engine security update. Actual package size, elevation/per-user behavior and disconnected launch must be measured.
- **Minimum API:** Runtime floor must support the selected print interface (WebView2 docs introduce `ICoreWebView2_16` printing in the 1.0.1518.46 line); set Tauri's minimum WebView2 version only after the pinned SDK/runtime and Windows proof are known. Do not claim that a version number alone proves all APIs work.
- **Data:** Keep clinic files in `%LOCALAPPDATA%` outside Program Files. Ordinary uninstall preserves data; scheduled-task registration is removed safely. Reinstall/upgrade and explicit destructive purge are separate workflows.
- **Publisher/signing:** No legal publisher entity or certificate was supplied. Do not invent either. Unsigned behavior/SmartScreen risk must be stated if no owner-provided certificate is configured. Signing material, if later used, lives only in protected CI secrets and is never requested/stored in chat.

### Offline runtime behavior

- Install the offline Evergreen runtime only when required; launch must succeed with network access disabled on a fresh supported Windows image.
- All product resources (JS/CSS/fonts/templates/icons) are local. No app update endpoint, external font, cloud login, analytics, ads, remote crash upload or runtime API is permitted.
- WebView2's Evergreen servicing is an OS/runtime lifecycle and may contact Microsoft when connected. Document that separately from DCMS Pro application traffic and test full offline functionality with the runtime already installed.
- If the offline installer cannot be installed under the supported non-admin policy, do not fall back to online bootstrapper silently. Reopen ADR-0001 and evaluate fixed runtime, installation mode or alternate stack with an explicit owner-facing size/security tradeoff.

## 2. Proposed CI workflow topology

The production workflow topology below is deferred until a real app/build manifest exists; no production job below has run. The separate isolated proof workflow has not yet run and does not build/release the application.

### Pull request / Arena branch CI

```text
frontend-static (Ubuntu)
  npm ci → lint → typecheck → component/unit → production frontend build

rust-domain (Ubuntu)
  pinned Rust → fmt --check → clippy -D warnings → domain/unit
  + bundled SQLCipher smoke (build, wrong key, backup) where platform-neutral

windows-msvc (pinned Windows runner)
  pinned Node + Rust MSVC + NSIS prerequisites
  → npm ci → frontend checks
  → cargo fmt/check/clippy/tests for x86_64-pc-windows-msvc
  → SQLCipher + DPAPI integration tests
  → Tauri NSIS production-profile build
  → package/notice/secret checks + retained artifact

windows-desktop-e2e (Windows GUI-capable runner, proof required)
  launch real Tauri/WebView2 app
  → IPC/RBAC/DB workflow smoke
  → local preview + PDF stream/metadata/text tests
  → printer enumeration/dialog or virtual driver tests where runner permits
  → collect safe logs, OS/WebView2 versions and test artifacts

security-supply-chain
  locked dependency/SBOM/license/advisory scan; secret and remote-resource scans
```

- Use `permissions: contents: read` for CI unless a job has a reviewed need; release publication permissions are separate. Pin third-party Actions by full commit SHA, use explicit minimal token permissions, avoid `pull_request_target` for untrusted code, and do not expose signing/activation secrets to PR builds.
- Pin Node LTS, Rust toolchain, Cargo/npm lockfiles, Tauri CLI, NSIS prerequisites, WebView2 SDK bindings and actions. No `latest`, unpinned installer download or floating CI action in release builds.
- Required gates fail closed: no `continue-on-error`, `|| true`, skipped tests or artifact publication after a failed prerequisite. Cache package downloads by lockfile; caches are untrusted acceleration, never secret storage.
- Store synthetic DBs, screenshots and PDFs only as intentional test artifacts, never as app seed data or release content. Retain failure evidence with safe redaction; keep `node_modules`, Cargo `target`, temporary outputs and runtime caches out of Git.
- PR CI must not receive `DCMS_ACTIVATION_VERIFIER`, signing credentials or a recovery identity. Release builds without owner-authorized activation verifier/signing inputs fail or explicitly use the unsigned posture; raw activation code is never a build input/log/artifact.

### Windows runner selection

- Do not use `windows-latest` as an implicit stable E2E environment. Pin a documented image. A September 2026 report describes WebView2 WebDriver session creation failing on `windows-2025` (runtime 152) while the same flow passed on `windows-2022`; treat this as a known runner risk and verify the issue before implementation. Current plan: run WebView2/Tauri E2E on an explicit `windows-2022` runner while separately compiling on a newer Windows image if supported, then revisit when upstream issue is resolved.
- CI runner is not the customer OS: hosted Windows Server cannot prove a clean Windows 11 install, interactive printer dialog, clinic printer, physical page quality or non-admin user workflow. Keep Windows 11 isolated VM/machine acceptance as a separate release gate.

## 3. Release pipeline (future implementation)

1. Build from a verified signed/unsigned version tag and exact commit on a clean Windows runner; release branch protections/environment approval are required.
2. Re-run all required CI jobs against the same locked dependencies and release profile; no source mutation after tests.
3. Build NSIS `.exe` with offline WebView2 installer, reviewed app icon, product version/metadata, production-only features and no devtools/source maps/test user/mock seed/activation bypass.
4. Scan final installer contents: app binaries, SQLCipher/OpenSSL/WebView2 notices, fonts/licenses, no secrets, private recovery identity, test DB, debug logs, unnecessary debug symbols or unexpected executables.
5. Generate SHA-256 checksum and SBOM/third-party notices; retain exact artifact, test results, OS/runtime versions and commit. If signing certificate is available, sign/timestamp using protected CI secret and verify signature; otherwise describe unsigned SmartScreen posture, never fabricate publisher identity.
6. Install exact artifact on a clean disconnected supported Windows 11 x64 VM/machine with no developer tools/source; perform setup/login, authorization, patient/visit/prescription/invoice/payment/stock, Bengali PDF/print, backup/restart/restore and uninstall/data-retention acceptance.
7. Publish GitHub Release only if actual repository permissions allow, with tested installer/hash/notes/notices. If publication fails for verified platform/permission reasons, follow Phase 0 `dist/` fallback and clearly state no GitHub Release was created.

## 4. Dependency selection and license evidence

No package is selected by name/version as a final lock yet. Candidates below require exact-version maintenance, vulnerability, transitive-license and binary-distribution review after proof builds.

| Component | Candidate | Initial license/evidence | Gate / restriction |
|---|---|---|---|
| Desktop shell | Tauri 2 + Wry | Tauri project components use permissive licensing; inspect exact Cargo package tree | Pin minor/patch; audit bundled Windows runtime and any plugins. No broad permissions. |
| UI runtime | React + React DOM + TypeScript + Vite | React MIT; TypeScript Apache-2.0; verify exact packages | Local-only bundle. Lock npm package graph; no remotely loaded scripts/assets. |
| Rust DB access | `rusqlite` + bundled SQLCipher + vendored OpenSSL | Rusqlite/libsqlite3-sys permissive; SQLCipher Community BSD-style with user-accessible notices; OpenSSL current license must be verified | Exact bundled source, cipher ABI, OpenSSL license/dependencies, MSVC build and wrong-key/backup proof. No plaintext fallback. |
| Password hashing | RustCrypto Argon2id/password-hash | Candidate RustCrypto permissive licenses | Use maintained API; OWASP baseline m=19 MiB/t=2/p=1 then benchmark; no custom KDF. |
| Portable encryption | Rust `age` crate, X25519 streaming | Crate currently lists MIT OR Apache-2.0 | Check exact version, keys/format interoperability, stream size/memory, notices and recovery tests. |
| Windows interop | Tauri `with_webview`, `webview2-com`, `windows`/`windows-sys` | `webview2-com` current docs list MIT; verify exact transitive licenses | Pin Tauri minor and bindings; inspect unsafe code; Windows UI/print/DPAPI proof. |
| Local fonts | Noto Sans / Noto Sans Bengali exact upstream files | Candidate SIL Open Font License 1.1 | Verify exact binaries/weight/script coverage, copyright, OFL text, Reserved Font Name/embedding terms; bundle notices. |
| React query/state/schema/testing | Candidate TanStack Query, React Testing Library, Vitest, Zod or equivalent | Exact licenses must be verified | Add only for demonstrated need; no persistent sensitive browser cache. |
| Icons/UI primitives | Custom icon + minimal accessible MIT/BSD headless primitives | Exact source/license to verify | No paid kit, unlicensed art, copied proprietary clinical glyphs or unused chart bundle. |
| Test PDF/image tools | Locally installed open-source PDF/text/raster libraries/tools | Exact licenses/version to verify | Test-only; do not ship or use cloud conversion. Validate that test rendering agrees sufficiently with WebView2; it is not output proof. |
| Installer/build runtime | NSIS, WebView2 Evergreen Offline Installer, MSVC redistributable if required | Microsoft/NSIS redistribution terms | Inspect exact installed payload, prerequisite licenses, elevation, update and offline-install policy. |

### Allowlist/policy

- Default candidates for review: MIT, Apache-2.0, BSD-2/3-Clause, ISC, zlib, CC0/public-domain and OFL-1.1 fonts, subject to each exact package's notice obligations.
- GPL/AGPL, paid/closed SDKs, subscription runtimes, source-available restrictions and licenses with network-service obligation are excluded unless legal review and owner explicitly approve the exact model.
- “Permissive” does not mean maintained, secure, suitable, or free of attribution obligations. Preserve full notices for SQLCipher Community and all native/OpenSSL/font/runtime components in an in-app legal/third-party notices page and installed documentation.
- Lock direct/transitive/build-time/test-only/runtime packages. Run SBOM generation (CycloneDX/SPDX candidate), `cargo deny`/license check, npm license/audit tooling, OSV/advisory review and final binary inventory. An allowlist change is reviewed, versioned and signed off; unknown license blocks release.
- Record source URL, package version, license text/hash, copyright, distribution form, direct/transitive status, vulnerability review and required notice file for every shipped component.

## 5. Test strategy mapped to architecture

| Level | Scope and oracle | Required environment |
|---|---|---|
| Static/type/security | TS lint/type, Rust format/clippy/unsafe review, secret scan, URL/network scan, license/advisory/SBOM | CI Linux + Windows; exact lockfiles. |
| Domain unit | Integer-poisha math/rounding, permission predicates, appointment/queue transitions, FDI, invoice/payment/reversal, stock, timezone, template snapshot normalization, error mapping | Rust tests; synthetic data only. |
| DB integration | Fresh DB/migrations/checksum/fk/unique/check/index, SQLCipher wrong key, WAL/reopen, writer/readers, backup API, corruption/disk-full/rollback | Exact Windows SQLCipher native build is authoritative; Linux can be additional evidence. |
| Service authorization | Invoke Rust use cases directly, forge IDs/roles/filters/cursors, assert denied projection/no DB change/audit | Rust integration with real temporary SQLCipher DB, no UI dependency. |
| React/component | Real interactions, controlled form validation, keyboard/focus, empty/loading/error, restricted data never cached | Vitest/Testing Library or selected equivalent. No production seed data. |
| Tauri/IPC E2E | Real app load, command registry/capability, login/session invalidation, local persistence/restart, no remote origin | Windows runner + clean Windows 11 acceptance; GUI automation path must be proven. |
| Backup/restore | Cryptographic envelope, correct/wrong recovery identity, integrity manifest, encrypted DB/attachments, fault-injected staged directory swap, notification | Windows integration with temp volumes; separate cross-profile test. |
| Document/PDF | Snapshot consistency, PDF dimensions/page count/text/font data/Bengali fixture, CSS pagination, totals/signature/tables | Actual WebView2 on Windows; test PDF inspector/rasterizer is an oracle aid, not replacement for visual review. |
| Printer | Enumeration, supported profile, selected printer, spool completion/error/cancel, A4/A5/thermal/Bengali | Windows driver and Microsoft Print to PDF if available; representative physical clinic printer required for final quality. |
| Installer/offline | Offline Evergreen provisioning, non-admin/per-user behavior, launch disconnected, upgrade/reinstall, shortcut/uninstall/data retention | Reset isolated supported Windows 11 VM/machine; no dev tools/source. |
| Usability/accessibility | Keyboard-only paths, focus, reduced motion, high contrast, resolution/scaling, Bengali content, empty/error states | Actual Windows screenshots/voice-over/manual human review; browser viewport alone is not proof. |
| Load/performance | Phase 0 synthetic workload and p95 targets for search/lists/profile/start/document, backup/restore time/memory | Named Windows 11 x64 4-core/8GB/SSD baseline and lower-end machine if available. Test volumes are not shipped. |
| Release acceptance | Exact installer hash, signature/posture, notices, feature journey, restore and print | Isolated Windows clean install with recorded OS/runtime/printer/driver/commit/hash. |

### Performance workload to carry forward

Use Phase 0 provisional workload: 100k patients, 500k visits, 1M events, 250k prescriptions, 250k invoices, 500k payment/allocation/reversal rows, 100k appointments, 100k stock movements, 100k audit events, 50k attachment records plus up to 5 GB synthetic files. Initial p95 budgets include patient search ≤500 ms at 100k, normal indexed list ≤800 ms, profile slice ≤1 s, typical 1–2 page prescription PDF ≤3 s, cold shell ≤5 s; confirm on named supported Windows hardware before tuning. Backup/restore budgets are unmeasured and must be set after Windows performance tests. Security/integrity is never relaxed for speed.

## 6. Requirement-to-design coverage

Phase 0 traceability remains normative. Phase 1 design maps directly to:

- **DB-001..004 / SEC-004..010 / BILL/PAY/ACC:** [`data-model.md`](data-model.md), [`security-and-rbac.md`](security-and-rbac.md), [`ipc-and-state-contract.md`](ipc-and-state-contract.md)
- **BACKUP-001..004 / OPS-003:** [`backup-and-recovery.md`](backup-and-recovery.md), ADR-0004
- **DOC-001..007 / RX / REL-004:** [`documents-printing-fonts.md`](documents-printing-fonts.md), ADR-0003
- **REL-001..008 / OPS-001..004:** this document, CI/release sections and [`architecture-review.md`](architecture-review.md)

No `acceptance-matrix.csv` planned case changes from `Not Run` are authorized here because no implementation or Windows acceptance ran.

## References

- Phase 0 test/release plan: [`../phase-0/test-and-release-plan.md`](../phase-0/test-and-release-plan.md)
- Phase 0 implementation phase gates: [`../phase-0/implementation-plan.md`](../phase-0/implementation-plan.md)
- Tauri Windows installer modes/size: <https://v2.tauri.app/distribute/windows-installer/>
- Tauri `with_webview` API: <https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html>
- Tauri capability security: <https://v2.tauri.app/security/capabilities/>
- SQLCipher Community license: <https://www.zetetic.net/sqlcipher/community/>
- Microsoft offline WebView2 deployment details via Tauri: <https://v2.tauri.app/distribute/windows-installer/>
- GitHub hosted Windows runner/WebView2 WebDriver report (recheck before CI adoption): <https://github.com/actions/runner-images/issues/14738>
- Microsoft .NET custom SQLite bundles: <https://learn.microsoft.com/en-us/dotnet/standard/data/sqlite/custom-versions>
- SQLCipher .NET licensing: <https://www.zetetic.net/sqlcipher/sqlcipher-for-dotnet/>
