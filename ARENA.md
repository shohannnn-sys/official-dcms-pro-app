# DCMS Pro — Arena Project Memory

**Last updated:** 2026-10-03 (Asia/Dhaka).<br>
This file is the durable handoff record. Verify Git/GitHub state before resuming; this file is not a substitute for repository state. Update it at every phase handoff and never claim a test or acceptance result without observing it.

## Product and owner constraints

- **Product:** Offline-first Windows desktop dental-clinic management system for Bangladesh; English UI by default, Bengali Unicode clinical entry/output, BDT/৳.
- **Privacy/security:** Protect clinical and financial data; enforce authorization in Rust/services, not only in UI; audit sensitive actions; preserve clinical/financial history. No hidden network calls, cloud dependency, paid/recurring runtime, telemetry or analytics.
- **Secret hygiene:** The supplied activation code must not appear in source, logs, docs, fixtures or artifacts. Do not request/store GitHub or owner credentials in chat. No production key, activation verifier, recovery private identity or signing secret is in this repository.
- **Execution order:** Follow `docs/phase-0/implementation-plan.md`; no production feature work in Phase 1. Architecture spikes are isolated and non-shipping. At each phase boundary report actual files, tests/results, defects, risks, acceptance, branch/commit, PR and exact next task; stop for the owner’s next **Continue**. Do not merge PR #1.

## Current phase and continuation

- **Current phase:** Phase 1 — Architecture and Technical Design, authorized by the owner's `Continue`.
- **Phase 0:** Documentation/discovery report is `docs/phase-0/completion-report.md`. Acceptance cases remain plans (`Not Run`); no app, installer, Windows or printer acceptance is implied. Original Phase 0 content commit: `caa87a092d836360a4445ce7adbed8f51ee346d8`; PR-status follow-up: `b17bc90cc341487979fd82a583af65d7f8903ee1`.
- **Phase 1 state:** Design package and architecture review are drafted and read back. ADR-0001..0004 remain **provisional/unlocked**. A non-shipping Tauri/Rust Windows spike and dedicated GitHub Actions workflow are drafted under `spikes/phase-1-windows-proof/`; the workflow has not yet run. No production UI, production schema/migration, app installer, activation verifier, recovery key or Phase 2 design system exists.
- **Exact next task:** Commit and push the Phase 1 package on the required Arena branch, inspect PR #1/Actions, run the Windows proof, capture and commit its generated `Cargo.lock`, fix genuine build/test defects, rerun with `--locked`, then record exact run URL/conclusion and artifacts. Do not treat workflow success as Phase 1 acceptance. Offline runtime installation/clean Windows 11, physical printer/Bengali visual output, cross-profile/portable recovery, `age` restore and owner confirmations remain separate gates. If unavailable, document the limitation and keep affected decisions unlocked.
- **Stop rule:** Do not begin Phase 2 or production implementation. At the Phase 1 handoff, report evidence and blockers, then wait for **Continue**.

## Repository and GitHub state

- Repository: `shohannnn-sys/official-dcms-pro-app` at `/home/user/official-dcms-pro-app`.
- Required branch: `arena/01a0fe63-official-dcms-pro-app`; never switch, create or push another branch. Session's starting Phase 1 HEAD was `eeea57f7e8427df79770ee468065ab9cef0370ec`; re-run `git branch --show-current`, `git rev-parse HEAD`, and `git status --short` after this work is committed/pushed.
- Original branch base: `f1a4e0a9469c618a10c25a44ddd0a3e797ba52de` (`Initial commit`). `main` began at the same baseline.
- **PR #1:** <https://github.com/shohannnn-sys/official-dcms-pro-app/pull/1>, open from the required branch to `main`, not merged. At the last query before Phase 1 push it had no reported status checks; re-check after push. Owner retains the manual merge decision.
- Never erase/rewrite the starting history; do not merge, force-push or modify another branch.

## Architecture candidates and unresolved owner gates

- Leading candidate remains Tauri 2 + React/TypeScript + Rust + local SQLite/SQLCipher, Rust-centered command/RBAC/audit boundary, DPAPI CurrentUser local key wrapper, WebView2 print/PDF, NSIS with offline Evergreen WebView2 runtime, and portable `age` encrypted backup. These are **proposals**, not accepted decisions.
- Scope assumption is one local clinic on one designated Windows user profile; no live multi-PC/LAN database or cloud sync. Owner confirmation is required before architecture lock.
- Recovery-key loss may make off-profile backups unrecoverable. Owner must approve the offline private-key export/retention UX; no key exists yet.
- SQLCipher/DPAPI same-user and cross-profile behavior, SQLCipher online backup, Tauri/WebView2 PDF, physical printer output, Bengali visual shaping/fonts, offline runtime installation, clean Windows 11 install, portable `age` restore, license/SBOM and performance are unproven. Documentation and compilation alone do not pass these gates.
- No publisher identity/signing certificate, printer model, minimum hardware policy, exact Bengali font binaries/license, local medical retention/consent or tax policy was supplied. Do not invent one or make legal/compliance claims.

## Files to use

- `docs/phase-0/` — requirements, risks, architecture candidates, permission/acceptance matrices, implementation and release plans; requirements baseline.
- `docs/phase-1/` — four provisional ADRs; system, IPC/state, logical ERD/data dictionary, security/RBAC, backup/recovery, documents/printing/fonts, Windows CI/dependency/license/tests, proposed folder structure, architecture review and status report.
- `spikes/phase-1-windows-proof/` — proof-only Cargo crate, synthetic English/Bengali HTML, SQLCipher backup and DPAPI tests, PDF verifier. Never copy into product source or release assets.
- `.github/workflows/phase-1-windows-proof.yml` — pinned `windows-2022` proof job; needs actual Actions run and reviewed artifacts.
- `README.md` and this file — project orientation and durable status.

## Environment and observed limits

- Sandbox is Linux x86_64, not Windows. Node 22.22.3, npm 10.9.8 and Python 3.11 are available; no `rustc`, Cargo, rustup, rustfmt, PowerShell, Windows, WebView2 or physical printer was available at Phase 1 review.
- A direct request to `static.rust-lang.org` failed TLS in this sandbox; no local Rust installation was possible. `gh` authentication/API access works; do not request credentials from the owner.
- GitHub-hosted Windows Server 2022 can supply a narrow API proof, not a clean Windows 11 customer install, offline Evergreen provisioning, other-profile recovery or physical paper-quality evidence.

## Checks and evidence ledger

### Phase 0

See `docs/phase-0/completion-report.md`: requirement/traceability/acceptance cross-check passed (109 requirement IDs, 109 traceability rows, 109 acceptance-plan rows and 545 linked planned case IDs; planned acceptance statuses remain `Not Run`); CSV parsing and Markdown links passed; activation plaintext scan passed; documentation `git diff --check` passed. No product tests/build were run.

### Phase 1 (before the first Windows workflow run)

- Local Markdown relative-link check: passed.
- Python `py_compile` for `scripts/verify_pdf.py`: passed; syntax only. Generated cache removed/ignored.
- Python TOML parse for `Cargo.toml` and JSON parse for `tauri.conf.json`: passed; syntax only.
- `git diff --cached --check`: passed across all 31 staged files after final documentation updates.
- Action commit SHAs checked through GitHub API for checkout, setup-rust-toolchain and upload-artifact.
- `cargo`, `rustc`, `rustup`, `rustfmt` and `actionlint`: unavailable locally. No Rust compile/test, SQLCipher, DPAPI, WebView2/PDF, Windows workflow, printer, installer or `age` backup/restore result exists yet.
- No test failures are known because the Windows workflow has not run; absence of a run is not a pass.

## Release/PR state

- There is no production app, product build, installer, release binary, app SBOM or release. No commit/merge/push of this Phase 1 package is recorded yet; update exact current SHA and PR checks after push.
- PR #1 is not to be merged by the agent. Never describe a hosted proof artifact as a product release.
