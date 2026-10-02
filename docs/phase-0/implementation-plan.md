# DCMS Pro — End-to-End Phase Plan

**Status:** Phase 0 discovery plan, dated 2026-10-02. This is a sequence and set of gates, not evidence that later work exists. No next phase may start until the owner says **Continue** after the prior phase report.

## Project-wide execution rules

1. At the start of each phase, inspect the current branch, clean/dirty state, recent commits, handoff, phase documents, tests and actual unfinished work. Never restart a complete module on the strength of conversational memory.
2. Work only on `arena/01a0fe63-official-dcms-pro-app`. The session is branch-bound. Use coherent PRs where appropriate, review the complete diff, and never merge; the owner controls merge decisions.
3. Implement only the current phase. Do not silently reduce a requirement or put a non-working control in the product. If a real technical defect appears, investigate, fix and retest within the phase.
4. A phase passes only with implementation + integration + data/authorization integrity + error handling + visual/accessibility review where applicable + tests + documentation + acceptance evidence. A compile or happy-path screenshot alone is insufficient.
5. Run the current phase's suite and regression for all completed phases; report commands, actual test totals/results and every unverified Windows/hardware item. Do not fabricate results.
6. Review the full diff for accidental assets, secrets, debug code, fake data, dead routes, temporary files and requirement gaps. Update `ARENA.md` with exact status, defects, schema/build/test state, PR state and next action.
7. Produce the structured completion report (phase, work/files/schema, tests passed/failed/not run, defects, known issues, criteria, branch/commit, PR/merge status, exact continuation task), then stop and wait for **Continue**.

## Phase gates and definitions of done

### Phase 0 — Product discovery and requirements freeze (current)

**Deliverables:** Repository/environment baseline; complete requirements specification with stable IDs and scope assumptions; full requirements-to-module/entity/screen/test/acceptance traceability; architecture candidate comparison and justified recommendation; proposed permission matrix; risk/dependency/license register; complete test/release plan; phase-by-phase implementation plan; persistent handoff memory.

**Definition of done:** Every original product area is represented in the specification and trace matrix; explicit conflicts/assumptions are recorded; candidate stacks are evaluated against offline install, Windows print, SQLite/security, deployment, performance and licensing; Windows/Linux constraints and unavailable evidence are stated; generated CSVs are parseable and IDs reconcile; documents are internally consistent and contain no activation plaintext; no production feature code has been started. Then report and stop.

**Current work:** Writing/validating `docs/phase-0/*` and `ARENA.md`. No application implementation.

### Phase 1 — Architecture and technical design

**Scope:** Re-inspect Phase 0 state; prove/decide Windows printing and offline WebView2 installation; prove selected encrypted SQLite build, DPAPI key protection and portable backup recovery; lock frontend/backend/native architecture in ADR(s); define OS/support model; module architecture and trust boundaries; state management/IPC contract; full normalized ERD and data dictionary/delete/index/transaction rules; permission matrix/key catalogue; activation/password/session design; backup/restore format; document/print and font design; installer/signing/release-workflow plan; dependency/license policy; automated/manual test strategy; final folder structure; architecture threat review.

**Definition of done:** Blocking spikes run on Windows or explicitly documented as unable to pass (in which case stack/security plan is not locked); ADRs state decision/context/alternatives/consequences; ERD covers all persisted concepts and history; every UI-to-native use case has a trust/permission boundary; recovery/activation/retention and one-PC assumption are explicit; all required dependencies have provisional license evidence; diagrams and test gates reviewed; no feature implementation begins. Update memory and stop.

### Phase 2 — Design system and UI/UX foundation

**Scope:** Original DCMS Pro visual identity/icon source and Windows ICO; local fonts and license notices; colors/type/spacing/grid and interaction tokens; shell/header/sidebar; typography, buttons, inputs/selects/date controls, tables, cards, modal/drawer/tabs/breadcrumb/tooltip, confirmation, notification, skeleton/empty/error primitives; keyboard/focus/reduced motion; responsive layout; non-production UI playground only if clearly marked as internal tests. No dead production nav items.

**Definition of done:** Components are used in real shell screens/routes rather than an isolated mockup; interactive states and validation behavior work; icon is visually inspected at packaged Windows sizes and has no white corners/cropping; local fonts render Bengali in live UI; responsive and DPI review includes requested dimensions/scales as the available Windows environment allows; automated UI/component/accessibility checks and manual visual audit pass; no fake product data shown as real. Update memory and stop.

### Phase 3 — Database, security and core infrastructure

**Scope:** Local app-data layout; encrypted SQLite if Phase 1 gate passed; versioned migrations; repositories/services; validation/error types/log redaction; session/auth/Argon2id; RBAC/default policy/deny tests; audit chain/append-only controls; auto-lock/draft strategy; Windows DPAPI/key storage; safe attachment storage base; settings; safe background work; DB backup primitives/telemetry-free network policy.

**Definition of done:** Fresh DB initializes; migrations and any supported upgrade path pass; foreign keys/checks/indexes/transactions verified; credentials/verifier are never plaintext; permission tests call the services without UI and deny forged IDs/requests; lock/restart/failed-auth paths work; DB corruption/wrong-key/locked/disk-full errors are recoverable and not false successes; audit state is durable and scoped; security docs and data tests pass. Update memory and stop.

### Phase 4 — Clinic setup and application shell

**Scope:** First-run activation screen (salted verifier, no code leak/bypass), recoverable multi-step setup, clinic/branding/hours, dentist profiles with multiple qualifications/designations/schedules, first owner login, baseline roles/settings, shell, real sidebar/header, notification entry point, About content.

**Definition of done:** Activation is enforced before setup/product data; valid/invalid input and repeated launch tested; interrupted setup resumes with preserved validated input and no partial usable clinic; back/next/edit/confirm work; all requested dentists/settings persist; shell opens only after atomic setup; restart/reopen works; About contains only authorized supplied details; every visible item navigates to a real permitted screen or is absent; tests, error/empty/loading states and UI audit pass. Update memory and stop.

### Phase 5 — Patient and clinical system

**Scope:** Patient registration/list/profile, unique codes, filters/search/date ranges; complete longitudinal profile/timeline; visits and templates; FDI adult/pediatric chart, surfaces/conditions/history; treatment catalog and records; referrals; attachments/file picker/preview/delete; clinic clinical history controls.

**Definition of done:** Patient/visit/chart/treatment/referral/attachment relations survive restart; repeated visits do not overwrite history; all profile totals/visit counts are real; uniqueness and permission paths are tested; file validation/path traversal/missing-file and authorized deletion audited; adult/pediatric tooth counts/numbers and per-visit historical states verify; no arbitrary row cap; large-data tests meet initial performance budgets; no unauthorized money leaks. Update memory and stop.

### Phase 6 — Appointments and queue

**Scope:** Day/week/month/list screens, appointment filters/status history/reschedule/cancel/no-show, dentist hours/overlap policy, patient-context creation, arrival/queue transitions, real elapsed waits and queue notifications.

**Definition of done:** Scheduled versus actual arrival/no-show is distinct; every transition is persisted, timestamped, attributed and auditable; invalid/repeated/concurrent transitions are safe; overlap warnings/overrides follow clinic policy; queue keyboard workflow is fast, no unnecessary modal dependency, and filters genuinely change results; permissions and failure states pass; regression of patient workflows passes. Update memory and stop.

### Phase 7 — Prescription engine

**Scope:** Prescription draft/finalization, medicine templates and custom multi-line orders, complaint/examination/advice, clinician/clinic content, versioned persisted snapshots, responsive A4/A5/thermal/mini paper profiles, preview, printer selection, Windows print/PDF adapter, local fonts/Bengali.

**Definition of done:** Real persisted prescription renders identical selected data in preview/PDF/print; multiple medicines and custom long Bengali/English content wrap; signature space remains sufficient; A4/A5/narrow/custom widths paginate intentionally and no output clips; physical installed printer plus Windows PDF path pass where hardware is available; printer unavailable/cancel/save failure recovers; generated PDFs are programmatically and visually inspected on Windows; authorization, audit/history, document tests and Phase 5/6 regression pass. Update memory and stop.

### Phase 8 — Billing, invoice, payments and financial system

**Scope:** Treatment charges, immutable invoice line snapshots, unique numbering, discounts/configured tax, partial/full/unpaid, payment allocation and reversals, wallet/bank methods, financial permission separation, income/expenses/categories/reports, invoice document and receipts if specified by ADR.

**Definition of done:** All math uses exact integer poisha and reconciles; invoice prices never mutate after catalog changes; payment and invoice operations are transactional; allocation/refund/void/overpayment invariants pass; unauthorized user cannot obtain financial values through service, route, search, export, dashboard or notification; invoice A4/A5/narrow print/PDF, Bengali, page totals and no-signature baseline visually pass; clear errors and financial reports match independent expected calculations; previous clinical/patient work regresses. Update memory and stop.

### Phase 9 — Inventory, staff and administration

**Scope:** Inventory, suppliers, batches/expiry/stock movements/costs, negative-stock policy and notifications; staff profile and salary/PII access; separate accounts; configurable roles/permission editor; operational settings and authorized audit views.

**Definition of done:** Stock equation reconciles from movements, negative policy works, all adjustments are audited; expiry/low-stock alerts derive from persisted values and deduplicate; custom roles test allowed and denied cases; staff PII/salary and user accounts remain isolated; settings persist and validate; all list/report filters work; load/error/empty/security states and regression pass. Update memory and stop.

### Phase 10 — Backup, restore, search and system hardening

**Scope:** User-selected backup folder, consistent database/files/config export, manual and scheduled catch-up backup, retention, checksum/encryption/schema validation, pre-restore backup and staged atomic restore; advanced global search; real permission-aware notification center; destructive-action safeguards; recovery/maintenance actions.

**Definition of done:** Backup works with app restart and attachment content, plus corrupt/wrong-key/incompatible/missing-folder/locked-file/disk-full cases; restore verifies before switch, makes pre-restore copy and preserves current DB on every injected failure; completion/failure notification is real; scheduler behavior while app is closed/off is documented and Windows-tested; search indexes/filtering/per-result authorization hold; restore/deletion cancel paths are no-change; recovery/test suites and previous feature regressions pass. Update memory and stop.

### Phase 11 — Integration, polish and full regression

**Scope:** Complete patient journey from registration through appointment/arrival, visit, dental chart, treatment, prescription, invoice, payment, follow-up and historical profile; repeated visits; multiple dentists/users/roles; lock/unlock; notifications/search/attachments; all docs; shell and every route/action; final keyboard/accessibility/high-DPI/visual audit.

**Definition of done:** End-to-end workflows run on Windows against persistent production-path services; all navigation/forms/buttons/filters/back/cancel/print/save/delete are exercised; one unauthorized negative workflow per permission family and destructive cancel tests pass; no fake/demo data; every requested screen has populated/empty/error/loading behavior; 1024×768 through 3840×2160, requested Windows scaling coverage and actual monitor screenshots reviewed to practical extent; regression suite clean; defects classified and critical/high defects fixed. Update memory and stop.

### Phase 12 — Security, performance, dependency and license audit

**Scope:** Threat review; auth/activation/DPAPI/SQLCipher, authorization, command/IPC, path/SQL/file/backup, audit, destructive action review; dependency and bundled-runtime SBOM/license/vulnerability audit; static secret/network/telemetry/mock/TODO/dead-code scans; realistic volume/performance/backup/PDF tests; notices.

**Definition of done:** No known critical/high finding; every medium finding fixed or explicitly accepted with owner; all source/transitive/build/runtime/font/icon obligations are recorded and required notices ship; no plaintext activation/password/private key/test account/network tracking; offline runtime network audit passes; measured start/search/profile/list/backup/restore/PDF timings meet Phase 1 budgets on named Windows hardware; stress data is test-only and excluded from release; security/perf regression passes. Update memory and stop.

### Phase 13 — Release candidate

**Scope:** Production build candidate and version metadata; GitHub Actions on clean Windows runner; retained artifacts and checksums; installer branding/icon/signature posture; isolated clean-machine installation/activation/setup; full representative workflow and failure/recovery; print/PDF/backup/restore/restart/uninstall.

**Definition of done:** Release gate suite passes with actual test counts; exact tested source tag/commit builds the installer; install requires no developer tools or online runtime download; correct data persistence/uninstall policy, product metadata, icon, activation, Windows printer/PDF and backup workflows pass on clean Windows. Review installer contents and signatures/notices. No unknown critical/high failure. If clean Windows/printer evidence is unavailable, status remains blocked; do not call it an RC acceptance pass. Update memory and stop.

### Phase 14 — Final audit and release

**Scope:** Second complete requirements-to-implementation audit against IDs/traceability immediately before final build; close forgotten requirements; final regression; final artifact rebuilt from verified source; GitHub Release and notices/notes/checksum or verified `dist/` fallback; final acceptance and QA report.

**Definition of done:** Every mandatory requirement has evidence and acceptance status; all release gates passed on the same final source/artifact; clean installation and critical workflows rechecked; no critical/high defects or failing tests; dependency notices and release notes verified; final artifact independently checksummed/inspected; GitHub Release is real if claimed (otherwise fallback explained); reports state actual counts, limitations and manual owner merge/release decisions. Owner controls merge; no agent merge. Update project memory and stop.

## Cross-phase stop conditions

- Stop and report, without pretending completion, if a required Windows/physical printer/owner legal or signing item is unavailable at its gate; technical bugs alone are not a stop reason and must be investigated/fixed within phase.
- If architecture decisions diverge from the Phase 0 baseline, update ADRs, requirements mappings, risk register and acceptance tests before implementation continues.
- If the owner merges a PR, inspect the resulting branch/commit/diff/workflows before moving to the next phase.
