# Phase 1 architecture review and proof-gate register

**Review status:** Design cross-review complete at the document level; **no architecture is accepted or locked**. No application release code, production schema, Windows 11 acceptance or owner approval of the open assumptions exists.

**Review date:** 2026-10-02 (UTC)<br>
**Review basis:** Phase 0 requirements, architecture candidates, permission/risk/acceptance artifacts and all current Phase 1 proposals. This is an internal design review, not a security certification or legal review.

## 1. Reviewed package

- Four provisional ADRs: desktop/trust boundary; SQLCipher/DPAPI; WebView2 printing; portable encrypted backup.
- System/trust-boundary architecture; typed IPC/client-state contract; normalized ERD/data dictionary/transaction rules; RBAC/security/session/activation design; backup/recovery; document/font/print; Windows delivery/CI/dependency/license/test strategy; proposed folder structure.
- Phase 0 specification, traceability/acceptance matrix, permission matrix, risk register, test/release plan and phase plan remain normative for requirements and acceptance.

## 2. Review outcome

### Provisional design baseline

Tauri 2 + React/TypeScript + Rust + local SQLite/SQLCipher remains the **candidate**, not an approved stack. The trust boundary is appropriately Rust-centered: UI is untrusted; Rust derives the session actor, authorizes individual fields/queries/commands, owns transaction/idempotency/audit and calls narrow native adapters. The live database is local to a designated Windows user profile; no live LAN database or cloud sync is in scope.

The database, key/recovery, WebView2 print, offline installer/runtime, and portable backup choices remain **unlocked** until the blocking evidence and owner decisions below are complete. A package API or successful compilation is not a pass.

### Cross-document findings and corrections

1. **Project state and location:** Readback confirmed the Phase 1 drafts and README edit are in this repository checkout; no external `/home/user/docs/phase-1` tree is used as a source. README now must describe the non-shipping proof harness accurately.
2. **Date normalization:** The index and four ADRs were corrected to `2026-10-02 (UTC)`. Project memory may use `2026-10-03 (Asia/Dhaka)` as the local handoff date; these are distinct date conventions and must not be conflated.
3. **Print result semantics:** WebView2's documented direct `Print` statuses are `SUCCEEDED`, `PRINTER_UNAVAILABLE` and `OTHER_ERROR`; an overlapping operation is `E_ABORT`/`OTHER_ERROR`. No distinct direct-print cancellation status is asserted. A system dialog with no completion callback records only that it opened.
4. **ERD semantics:** Removed the misleading `users → staff_salary_records: can_read` relationship. Salary visibility is a permission decision, not a salary-row ownership relation.
5. **Secret hygiene:** The test spike uses fixed, clearly proof-only SQLCipher/DPAPI bytes and synthetic Bengali text; no supplied activation code, production activation verifier, recovery private key, signer or patient data is present.
6. **Unmeasured estimates:** The WebView2 offline installer size, performance budgets, runtime availability and printable output remain estimates/plans until measured in the final Windows environment.

### Required design clarifications before architecture lock

| Item | Current assumption | Required disposition |
|---|---|---|
| Clinic topology | One local clinic DB on one Windows PC; no concurrent workstations/LAN share/cloud sync | Owner confirms. If multi-PC sharing is required, stop and redesign rather than place SQLite on a network share. |
| Windows identity | One designated Windows user profile owns `%LOCALAPPDATA%` and DPAPI CurrentUser key wrapper; in-app users are distinct from Windows users | Owner confirms that other Windows profiles do not need live access. |
| Recovery UX | Owner exports an offline `age` X25519 private identity and stores it separately from backups; loss can permanently prevent recovery | Owner explicitly accepts the responsibility and tests export/open/challenge/cross-profile restore. If not, evaluate a different recovery model before approval. |
| Offline provisioning | Bundle Evergreen offline runtime in candidate NSIS installer | Clean disconnected install, including non-admin/elevation behavior, must pass. Otherwise reopen stack/install ADR. |
| Printer requirements | Installed Windows drivers only; no vendor-specific Bluetooth/USB protocol promised | Provide a representative clinic printer/driver, paper widths and OS build for final print acceptance. A hosted runner cannot prove physical paper output. |
| Supported hardware | Windows 11 x64 servicing release, reference 4-core/8-GB/SSD budget remains provisional | Owner confirms minimum CPU/RAM/storage and target Windows servicing policy; benchmark on the named baseline. |
| Commercial publisher | No legal publisher or certificate supplied | No publisher claim/signing identity is invented. Owner supplies authorized certificate through protected release infrastructure if signing is desired. |
| Legal/retention/tax policy | No Bangladesh-specific medical retention, consent, disclosure or tax rate supplied | Owner/local professional review; no legal-compliance claim or hardcoded tax rate. |
| Font | Noto Sans Bengali/Noto Sans candidate; no exact font binary/license copied yet | Verify exact upstream files, OFL notice/embedding rights, glyph coverage and Windows/PDF output before distribution. |

## 3. Blocking proof gates

| Gate | Evidence required | Evidence available now | Status |
|---|---|---|---|
| SQLCipher + MSVC | Exact locked `rusqlite`/SQLCipher/OpenSSL build on Windows; cipher version; wrong-key denial; no plaintext DB/WAL marker; WAL/reopen; same-key online backup | Isolated harness and pinned Windows workflow drafted; no Windows run yet | **Blocked / not run** |
| DPAPI CurrentUser | Protect/unprotect and tamper rejection in the same Windows profile; second-profile failure/portability behavior measured separately | Isolated Windows FFI test drafted; no Windows run; cross-profile behavior not tested | **Blocked / not run** |
| Tauri/WebView2 PDF | Real Tauri window, actual runtime, `ICoreWebView2_16`, A4 settings, PDF stream, PDF validity and English/Bengali/BDT text extraction | Isolated WebView2 proof app/workflow drafted; no Windows run yet; no raster/glyph review | **Blocked / not run** |
| Native printer/dialog | Enumerate installed printers, selected-printer `Print`, native dialog status, offline/unavailable/busy/error handling, supported paper geometry; physical clinic printer printout | Documentation and plan only | **Blocked / unavailable in current sandbox** |
| Evergreen offline installer | Build NSIS with Tauri offline runtime, install disconnected on clean supported Windows 11 without developer tools, launch, update/uninstall/data preservation | No installer/clean Windows 11 VM created | **Blocked / not run** |
| Portable recovery | `age` streaming, correct/wrong private identity, tamper/truncation, cross-profile restore, attachment checks and interrupted staged swap | Design only; no recovery harness/owner-held recovery key | **Blocked / not run** |
| Dependencies/licenses | Exact app lockfiles, native binary inventory, SBOM, notices and vulnerabilities | Candidate policy only; no product manifest or release binary | **Not applicable yet / not run** |
| Owner scope | One-PC/profile, owner-held recovery-key UX, Windows target and printer/hardware baseline | Assumptions recorded; no additional owner decisions in this review | **Pending** |

The hosted `windows-2022` spike can prove a narrow SQLCipher/DPAPI/WebView2 API path. It cannot substitute for a clean Windows 11 installer, a different user profile or a physical printer. No gate becomes accepted solely because a workflow is green.

## 4. Architecture risks and required safeguards

1. **Tauri/COM binding drift:** pin Tauri minor/patch and compatible `webview2-com`/`windows` versions; isolate unsafe code; run API/print tests after every upgrade.
2. **SQLCipher key interpretation:** prove raw-key syntax against the built cipher; call cipher-version check before first protected data access; no plaintext fallback.
3. **DPAPI's limited boundary:** CurrentUser protects against casual copying by another profile, not malware in the same profile, a local admin or a debugger. Maintain app RBAC and Windows account controls.
4. **Backup complexity:** Separate database key, local DPAPI wrapper and user-held recovery identity; use strict path/size validation, ciphertext-only staging, pre-restore verified backup, durable journal and fault-injection before activation.
5. **Printer observability:** Map only actual API statuses, distinguish dialog opened/spool accepted/physical page observed, preserve the finalized record on all print failures.
6. **Bengali quality:** Text extraction does not prove visual glyph shaping. Require bundled licensed font verification plus raster and physical review in the release gate.
7. **Authorization surface:** Every query projection, aggregate, search snippet, notification and document output is a separate protected surface; test services directly without UI.
8. **Schema complexity:** Treat Phase 1 ERD as logical only; do not generate production migrations until every persisted relation, deletion, money invariant, migration/restore version and permission projection is reviewed.
9. **Local scheduled task:** Never embed Windows credentials or secrets in task arguments. Same-profile DPAPI and app/task lock behavior must be demonstrated before unattended backup is enabled.
10. **Release/legal posture:** Final lockfiles, bundled fonts/runtime/SQLCipher/OpenSSL and package notices are release gates; do not claim FIPS, regulatory compliance, physical print, installer or SmartScreen outcomes without evidence.

## 5. Traceability review

- Core requirements map through [`system-architecture.md`](system-architecture.md), [`ipc-and-state-contract.md`](ipc-and-state-contract.md), [`data-model.md`](data-model.md) and [`security-and-rbac.md`](security-and-rbac.md).
- Backup/recovery requirements map through ADR-0002/0004 and [`backup-and-recovery.md`](backup-and-recovery.md).
- Document/printing/font requirements map through ADR-0003 and [`documents-printing-fonts.md`](documents-printing-fonts.md).
- Packaging/offline runtime/CI/license/release requirements map through [`windows-ci-dependencies-tests.md`](windows-ci-dependencies-tests.md).
- Proposed implementation code layout is [`folder-structure.md`](folder-structure.md).
- Phase 0 `acceptance-matrix.csv` remains unchanged with all planned statuses `Not Run`; this review authorizes no implementation acceptance claim.

## 6. Review decision

**Result:** Accept as a coherent *proposal for proof and continued Phase 1 work*, not as the final architecture. The proposed Tauri stack/security choices must remain **provisional** until the Windows proofs and owner confirmations pass. Do not start Phase 2 design-system implementation or Phase 3 infrastructure/production features before Phase 1 acceptance and a new owner `Continue`.

Review date, document links or code compilation do not substitute for evidence. Exact proof results, commands, defects and run identifiers belong in [`status-report.md`](status-report.md) and [`ARENA.md`](../../ARENA.md).

## References

- Phase 0 architecture candidate evaluation: [`../phase-0/architecture-candidates.md`](../phase-0/architecture-candidates.md)
- Phase 0 implementation plan and stop rules: [`../phase-0/implementation-plan.md`](../phase-0/implementation-plan.md)
- Phase 0 risk register: [`../phase-0/risk-register.md`](../phase-0/risk-register.md)
- Microsoft WebView2 `ICoreWebView2_16`: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/win32/icorewebview2_16>
- Tauri Windows offline installer options: <https://v2.tauri.app/distribute/windows-installer/>
