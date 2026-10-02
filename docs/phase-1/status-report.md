# Phase 1 — Architecture and Technical Design: status report

**Report status:** In progress; no Phase 1 acceptance decision yet.<br>
**Report date:** 2026-10-02 (UTC)<br>
**Current branch:** `arena/01a0fe63-official-dcms-pro-app`<br>
**Starting HEAD:** `eeea57f7e8427df79770ee468065ab9cef0370ec`<br>
**PR:** #1, open to `main`; do not merge.<br>
**Architecture:** Candidate proposals only; no ADR accepted/locked.

This report will be updated with any actual hosted Windows workflow run and exact final commit/PR state. It does not report future or unobserved results.

## 1. Work completed in this report cycle

- Integrated and reread the four provisional ADRs and supporting Phase 1 architecture/security/data/backup/printing documents already present in the checkout.
- Added proposed Windows delivery/CI/dependency/license/test strategy and proposed final folder structure.
- Added architecture cross-review and explicit owner/proof gates.
- Corrected Phase 1 index/ADR dates to `2026-10-02 (UTC)`.
- Corrected the ERD's misleading salary visibility edge and WebView2 print-status language.
- Added an isolated non-shipping Windows architecture proof harness and a minimal workflow for SQLCipher/DPAPI and a local Tauri/WebView2 PDF stream.
- Reconciled the top-level README and `ARENA.md` with the actual checkout state (to be maintained through final verification).

## 2. Files and architecture artifacts

Phase 1 documentation is under `docs/phase-1/`: index, ADR-0001..0004, system architecture, IPC/state contract, logical ERD/data dictionary, security/RBAC/session/activation, backup/recovery, Bengali documents/printing/fonts, Windows delivery/CI/dependency/license/test strategy, folder structure, architecture review and this report.

The proof harness is under `spikes/phase-1-windows-proof/`, with a dedicated `.github/workflows/phase-1-windows-proof.yml`. It is not application code and must not ship. There is no production schema or migration, React application, release installer, activation verifier, recovery key, patient fixture or Phase 2 design system.

## 3. Commands and test results

### Local checks actually run

| Command/check | Result |
|---|---|
| Repository branch/HEAD/status and PR #1 status | Inspected; branch/starting HEAD and open PR are recorded above. |
| `git diff --cached --check` | Passed across all 31 staged files after the final doc/report updates. |
| Python `py_compile` for the proof-only PDF verifier | Passed; this checks Python syntax only, not a generated PDF or Windows behavior. Temporary `__pycache__` output is ignored/removed. |
| Python TOML/JSON parse of proof manifests | Passed; syntax only. |
| `cargo`, `rustc`, `rustup` availability | Not available in this Linux sandbox. |
| Local app build, Rust tests, Windows runner, WebView2/PDF, printer, installer, DPAPI/SQLCipher tests | Not run locally; no claim of pass. |
| GitHub Actions checks | No result until the workflow actually runs; current PR previously reported no checks. Update with the exact run ID/URL, job conclusion and test counts. |

### Hosted Windows proof ledger

| Gate | Status before workflow execution | Result after workflow execution |
|---|---|---|
| MSVC compile / Rust test suite | Not run | **Pending; update only after observed run.** |
| SQLCipher cipher version, wrong key, ciphertext/WAL marker, same-key online backup | Not run | **Pending.** |
| DPAPI CurrentUser round trip and tamper rejection | Not run | **Pending.** Cross-profile behavior remains separate. |
| Tauri WebView2 `ICoreWebView2_16::PrintToPdfStream`, A4 metadata, English/Bengali/BDT text extraction | Not run | **Pending.** PDF extraction does not prove visual glyph quality. |
| Offline WebView2 installer / clean Windows 11 install | Not run | **Not part of this hosted proof; remains blocked.** |
| Physical printer, system dialog, printer enumeration, thermal profiles | Not run | **Not part of this hosted proof; remains blocked.** |
| `age` portable recovery and journalled restore | Not run | **Not implemented/run; remains blocked.** |

## 4. Schema, build and release state

- **Schema/migrations:** None. The ERD is a proposed logical design only.
- **Production package manifests:** None. The proof crate has exact direct version constraints; its generated `Cargo.lock` must be captured from its first Windows run and committed before the workflow is treated as reproducible.
- **Production frontend:** None.
- **Installer/build:** No NSIS package, offline runtime bundle, clean-install artifact or signed binary exists.
- **Release/PR:** PR #1 remains owner-controlled and is not to be merged by the agent. No release exists.
- **Dependency audit/SBOM:** Candidate license policy only; no product dependency graph or release binary was audited.

## 5. Defects, known issues and risks

- This sandbox has no Rust toolchain and cannot make the native Windows test results locally. The initial Windows workflow may expose compile/API/runner issues; fix within Phase 1 and rerun rather than weaken tests or mark them skipped.
- GitHub-hosted Windows Server 2022 is not a supported Windows 11 clean-install test machine. The app run only verifies the API path on the recorded hosted runtime.
- The proof does not rasterize/visually inspect Bengali, exercise a selected installed/physical printer, provision WebView2 offline, test another Windows profile, or perform `age` recovery.
- Final product package, secure installer, SQLCipher/DPAPI production API integration, activation build input, recovery UX, exact fonts, OS support policy, hardware baseline and owner acceptance are unresolved.
- No test failures are known because the Windows workflow has not yet been observed; absence of a run is not a pass.

## 6. Acceptance status and exact next action

**Phase 1 acceptance:** **Not met / in progress.** The architecture remains unlocked.<br>
**Immediate next action:** Run and review the pinned Windows architecture-proof workflow; fix any genuine compile/test defects; commit the generated proof `Cargo.lock`; rerun with `--locked`; then update this report and `ARENA.md` with the exact run URL, conclusions, evidence artifact and remaining failures. Continue the offline-runtime/install, physical-printer and portable-recovery gates or document their unavailable evidence without locking the decisions.

Do not begin production features, Phase 2 design-system work or Phase 3 infrastructure implementation. At Phase 1 boundary, report actual changed files/tests/defects/risks/acceptance/branch/commit/PR and stop for the owner's next **Continue**.
