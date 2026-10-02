# DCMS Pro — Arena Project Memory

This file is the durable handoff record for this repository. Verify the repository and these notes before resuming; never trust this file instead of the current Git state.

## Product and execution state

- **Product:** DCMS Pro, an offline-first Windows desktop dental-clinic management system for Bangladesh.
- **Current phase:** Phase 0 — Product Discovery and Requirements Freeze.
- **Phase 0 status:** Complete as a documentation/discovery phase; report: `docs/phase-0/completion-report.md`. No production application code, schema, dependencies or test scaffolding has been started.
- **Phase 1 status:** Not started. Do not begin it until the owner says **Continue**.
- **Exact continuation point:** On **Continue**, first inspect `git status --short --branch`, `git log -5 --oneline --decorate`, `git rev-parse HEAD`, the complete diff and all `docs/phase-0/` documents. Verify the completion report against actual repository state. Then begin Phase 1 by resolving the architecture gates in `docs/phase-0/architecture-candidates.md`, creating formal ADR(s), ERD, module/security/backup/document-printing designs and Phase 1 test/dependency plan. Do not implement production features during Phase 1. Stop at Phase 1's definition of done and wait again.
- **Date of handoff:** 2026-10-02 (UTC).

## Repository baseline (verified)

- Repository: `shohannnn-sys/official-dcms-pro-app`.
- Required working branch: `arena/01a0fe63-official-dcms-pro-app`. Never switch branches; Arena associates work with this branch.
- Starting commit: `f1a4e0a9469c618a10c25a44ddd0a3e797ba52de` (`Initial commit`), the same commit as `main` at inspection time.
- Initial tracked files: one file, `README.md`, containing only `# official-dcms-pro-app`.
- No application source, package manifests, Rust/Cargo project, database/schema/migrations, tests, documentation, GitHub Actions, installer, icons, license notices, or ignore rules existed at discovery.
- Initial Git worktree was clean. Repository remote `origin` points at the project GitHub repository.
- Do not erase or rewrite that starting history. Phase work is additive and must stay on the Arena branch. Do not merge a PR.

## Development environment observed at Phase 0

- Host: Linux x86_64 sandbox; this is **not** Windows.
- Available: Node.js 22.22.3, npm 10.9.8, Yarn 1.22.22, Python 3.11.8, GCC 12.2, GNU Make, Git, and `gh`.
- Not available at inspection: Rust/rustup/Cargo, .NET SDK/MSBuild, Windows/PowerShell/Visual Studio Build Tools, Wine, browser binaries, Playwright, or a physical Windows printer.
- No project dependencies are installed because the repository has no manifests.
- SQLite is available through Python's standard library; no SQLite CLI was found/claimed.
- Consequence: this sandbox can author/review documentation and later run suitable frontend/static tests after tooling is installed, but it cannot independently produce or validate a Windows installer, WebView2 printing, Windows high-DPI behavior, clean Windows install/uninstall, or real printer output. Windows CI and an isolated Windows acceptance machine are required for those claims.

## Architecture and scope notes

- Phase 0's leading technical recommendation is **Tauri 2 + React + TypeScript + Rust + local SQLite**, with all business rules, authorization, data access, audit and native operations on the Rust side.
- Candidate encryption is SQLCipher Community Edition for the local database, with Windows DPAPI protection for local key material; the precise SQLCipher build, portable backup recovery model, and legal notices must be proven in Phase 1 before lock-in.
- HTML/CSS in the local WebView2 renderer is the candidate shared document-layout engine. A narrow Rust/Windows WebView2 adapter would enumerate installed Windows printers and use WebView2's native print/PDF APIs. This is a **Phase 1 proof gate**, not implemented or validated yet.
- Product scope assumption: one clinic database per Windows PC, multiple dentists and in-app users, no LAN-shared live database, cloud sync, or mandatory internet. A user-selected Windows-installed printer/driver is the compatibility boundary; no direct vendor Bluetooth protocol is promised.
- See `docs/phase-0/architecture-candidates.md` and `docs/phase-0/requirements-spec.md` for complete candidate evaluation and resolved assumptions.

## Database and security status

- Schema status: none exists; no migrations or database files have been created.
- Schema direction: versioned SQLite migrations, foreign keys, deliberate delete rules, indexes, transaction-based use cases, historical clinical/financial snapshots, soft deletion where appropriate, and no application-level patient/visit/history count cap.
- Authorization direction: permission checks in Rust application services on every command and result query; the UI only mirrors permissions. Financial data and search previews are separately gated. Audit writes are part of the same business transaction where feasible.
- Activation: offline local gate, a salted Argon2id verifier rather than plaintext source, constant-time comparison, no code logging, and an explicit disclosure that a determined binary analyst can bypass local verification. The production activation code is intentionally not copied into this memory file.
- No credentials, activation verifier, production secrets, or private keys have been added.

## Test, build and release status

- Tests/checks executed in Phase 0 (see `docs/phase-0/completion-report.md`): repository/Git/file/toolchain inspection; one documentation consistency validator passed (109 requirement IDs, 109 traceability rows, 109 acceptance-plan rows, 545 linked planned case IDs, all planned status values `Not Run`, local Markdown links valid, activation plaintext absent); both CSVs parsed successfully; `git diff --check` passed after whitespace corrections. No application tests exist and no application test result is claimed.
- Build commands: not yet defined; there is no project manifest. Phase 1 will define them, and implementation phases must keep this file current.
- Release commands/workflows: not yet defined; `.github/workflows/` does not exist. The plan requires Windows GitHub Actions, failing test gates, a production Windows installer, checksums, retained artifacts, and a GitHub Release where permissions allow.
- Packaging constraint: Windows build, install, WebView2, print, icon and uninstall acceptance need Windows runners/machines. Release must not be described as clean-machine-tested until it has actually been installed and exercised on an isolated supported Windows environment.
- Phase 0 content commit: `caa87a092d836360a4445ce7adbed8f51ee346d8`; PR-status follow-up commit: `b17bc90cc341487979fd82a583af65d7f8903ee1`. Always read the exact current branch tip using `git rev-parse HEAD` on resumption.
- PR status: Phase 0 documentation PR [#1](https://github.com/shohannnn-sys/official-dcms-pro-app/pull/1) is open from the Arena branch to `main` and is not merged. The owner retains the manual merge decision; never merge. No Actions workflow existed at Phase 0, so do not imply application CI ran.

## Known decisions, risks and owner-facing gates

1. Current Windows support policy recommendation is Windows 11 x64 on a Microsoft-supported servicing release. Windows 10's general support ended 2025-10-14; adding it would require an explicit risk/scope exception.
2. The local single-PC/no-LAN interpretation is deliberate. If multiple workstations must share live data, stop before architecture lock; that is a materially different networked system.
3. A physical clinic printer/model is not available in this sandbox. Test installed Windows drivers (including Microsoft Print to PDF) in CI/Windows acceptance; validate at least one representative standard printer before final release if hardware can be provided.
4. No publisher legal entity or signing certificate was provided. Do not fabricate one. The final Windows trust/SmartScreen posture must be documented; signing can only be configured if the owner supplies an authorized certificate through GitHub secrets (never chat).
5. The owner has not specified local tax policy, paper printer models, medical-record retention/consent policy, or jurisdictional legal review. Defaults and release gates are documented; do not invent legal compliance claims or hardcode a tax rate.
6. The feature set is intended to be final, but essential security fixes cannot ethically be excluded from the release process. Treat feature freeze separately from security maintenance; record any decision to refuse future security updates as an owner-accepted risk.

## Project-memory update rules

At the end of every phase, update: current phase/status; exact next unfinished task; branch and commit; files/modules and schema changes; commands run and actual test counts/results; build/release state; defects found/fixed; open risks; PR/merge state. Never write “passed” or “complete” without evidence. Preserve earlier phase records (append a dated phase report rather than rewriting history away).
