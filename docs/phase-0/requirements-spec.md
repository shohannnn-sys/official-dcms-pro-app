# DCMS Pro — Phase 0 Requirements Specification

**Document status:** Discovery baseline for owner review; no application implementation has begun. **Baseline date:** 2026-10-02 (UTC). **Normative language:** “MUST” is an acceptance requirement; a traceability row and tests are required before it can be called complete.

This specification consolidates the owner's master directive, resolves ambiguities using the defaults below, and identifies boundaries that must not be silently expanded or weakened. `docs/phase-0/traceability.csv` maps every requirement ID to modules, data entities, screens, verification, and acceptance evidence.

## 1. Product purpose and fixed product facts

DCMS Pro is a commercial, offline-first Windows desktop application for a dental clinic in Bangladesh. It manages one practice with multiple dentists and staff accounts on a local Windows computer. It is not a cloud service, web-hosted system, multi-tenant service, patient portal, or shared-network database in this baseline.

| Topic | Frozen baseline |
|---|---|
| Product name | DCMS Pro. Product/about copy may identify the developer as **Shohan Khan**, `helloiamshohan@gmail.com`; no additional company claims or identity may be invented. |
| Audience | Owner/admin, dentists, receptionists, assistants/nurses, finance staff, inventory/store staff, and configurable roles. |
| Language | Application chrome and help default to professional English. Bengali Unicode is supported in user-entered clinical, patient, prescription, invoice, advice, address, name, and other free-form content. This baseline does not promise a fully translated Bengali UI. |
| Locale and time | Clinic business time zone defaults to `Asia/Dhaka`; persisted instants are timezone-aware and displayed in clinic time. Date-only values such as date of birth remain date-only. English Gregorian display is the default. |
| Currency | BDT only; monetary values are stored as integer poisha (100 poisha = 1 BDT) and displayed consistently with `৳` and two fractional digits. No foreign-exchange feature. VAT/tax is configurable but off unless the clinic owner configures it; DCMS Pro does not assert a tax/legal rate. |
| Connectivity | All normal workflows, activation verification, database access, local document/PDF generation, backup, restore, search, and printing through installed Windows drivers work without internet. No telemetry, ad/analytics SDK, cloud sync, remote database, paid API, or hidden application network call. User-selected backup destinations and Windows-managed printer services are explicit OS integrations, not application cloud services. |
| Record capacity | “Unlimited” means there is no product-imposed row count or patient cap. SQLite's theoretical upper database size is about 281 TB only at its maximum page size; real OS/filesystem/device/storage and performance limits are much lower, so DCMS Pro makes no infinite-capacity claim. Large binary attachments live in managed file storage rather than SQLite rows. Pagination/virtualization and indexed queries are mandatory. |
| Default deployment | One clinic / one local encrypted database / one Windows PC, with multiple in-app accounts and dentists using that installation. No live multi-PC/LAN sharing, synchronization, or multi-clinic tenancy is promised. Never put a live SQLite/WAL database on a network share. |
| Data storage | Candidate per-Windows-user root: `%LOCALAPPDATA%\DCMS Pro\` with database/key-protection metadata under `data/` and `security/`, opaque managed attachments under `attachments/`, and redacted diagnostics under `logs/`. The active database remains on a local disk; no cache/logs/secrets are part of a backup. User-selected backups/exports live only at a user-chosen destination. Ordinary uninstall preserves clinic data and activation state; explicit data destruction is separate, authenticated and confirmed. Phase 1 must validate exact Windows path/ACL behavior. |
| Supported OS recommendation | Windows 11 x64 on a Microsoft-supported servicing release. Windows 10 is not a supported baseline after its general support ended; Windows 10 execution is out of scope unless the owner explicitly accepts an unsupported-OS security exception. Windows on ARM is not a release target unless separately built and tested. |
| Installation | A non-developer can install the production Windows installer. No Node, Rust, Git, Python, SDK, compiler, package manager, or manually installed development runtime is required. Required WebView2 runtime must be present or bundled/installed offline by the installer. |
| Maintenance | Feature scope is intended to be final. The architecture and migrations must be complete at first production release. Security fixes remain a separate maintenance obligation; a promise never to issue a security fix is not implied. |

## 2. Scope boundaries and interpretations

### 2.1 Included

- Local clinic identity and branding; multiple dentist profiles, schedules, designations, qualifications and certifications.
- First-run activation and recoverable setup; local login, sessions, granular roles/permissions and lock screen.
- Dashboard, patients and longitudinal records, appointments, queue, visits, adult/pediatric dental charts, treatment catalog, prescriptions and attachments/referrals.
- BDT invoices, allocated payments, inventory, income/expense accounting, reports and financial access controls.
- Auditing, local notifications, global search, printer/document profiles, Windows print/PDF export, local backup/restore and settings.
- Versioned SQLite migrations, performance/error/accessibility testing, Windows Actions build, installer, notices, checksums and release acceptance evidence.

### 2.2 Explicitly outside the baseline (not implied by a visible control)

- Shared real-time use from multiple computers, LAN server/database, hosted/cloud services, cloud synchronization, patient mobile/web portal, or remote access.
- Online payment processing, bank/mobile-wallet APIs, paid integrations, OCR, automatic diagnosis, automated drug/dose recommendations, or claims that a template is medical advice.
- Direct Bluetooth/USB printer protocol implementations, printer firmware control, or vendor-specific discovery. Printing is via printers and drivers installed in Windows.
- Multi-branch/multi-clinic tenancy, insurance claims, payroll tax filing, electronic signature/certificate issuance, e-prescribing networks, or regulatory certification unless later expressly scoped.
- Full Bengali translation of all application chrome. Bengali input and rendering in documents remain in scope.
- Automatic external data import/sync. Any later import must be separately validated, authorized, previewed, and audited.

### 2.3 Assumption resolution / potential owner change request

| Ambiguity | Phase 0 resolution | Consequence if changed |
|---|---|---|
| “Multiple dentists” could mean multiple devices | Multiple dentist and staff accounts on one local installation; no live LAN sharing. | LAN use changes database, authentication, concurrency, backup and threat architecture; resolve before Phase 1 lock if required. |
| Windows version unspecified | Support current-servicing Windows 11 x64; do not endorse unsupported Windows 10 for patient data. | Supporting Windows 10 requires an owner-accepted security exception and separate runtime/installer testing. |
| Activation “during installation/initial activation” | First-launch activation screen runs before setup or production data access; no network request. Installer itself does not send or validate the code. | The local check is intentionally reversible by a determined binary analyst. It is a reasonable offline gate, not DRM assurance. |
| Activation code appears in the user brief | Do not reproduce it in repository documentation, tests, seed data, CI logs or source literals. Build only a salted one-way verifier. | Production verification must be tested without emitting code material or adding a bypass. |
| Tax/VAT policy | Support optional configurable invoice tax/adjustments; default disabled and no legal rate hardcoded. | Clinic owner/accountant configures the applicable values after local review. |
| Payment allocation and correction | A payment is a transaction separate from an invoice and may have one or more explicit invoice allocations; unallocated credit is visible only to authorized users. Corrections use reversal/refund/void records, not destructive history edits. | This preserves partial payments, reconciliation and auditability; test allocation sums and negative/reversal cases. |
| Multi-backup restore selection | Each `.dcmsbackup` is a self-contained point-in-time set; restore one selected backup at a time. | No ambiguous merge of multiple database snapshots. Selecting more than one file is not offered. |
| Automatic backup while app is closed | Due backups run at next app startup and, if Phase 1 proves safe, through a per-user Windows scheduled task/logon catch-up while the machine/user is available. Power-off periods are caught up next login; no guarantee is made while a machine is off. | Installer/task registration and user-folder permissions need Windows testing. Backup failure must become a real notification. |
| Printer support | Use the Windows print subsystem for installed, available drivers; default-printer, selected-printer, Save as PDF and direct PDF export are separate paths. | No claim of raw Bluetooth/USB support. Test hardware models when available. |
| Age / date of birth | Store nullable date of birth and, when only an approximate age is known, a reported age with date/context; do not fabricate a birth date. Historical document age can be snapshot/configured. | Avoid implying false precision and keep old printed records stable. |
| Clinical or financial deletion | Soft-delete or controlled void/retention workflows; do not cascade-delete clinical/financial history. Hard purge is an explicit, strongly authenticated, backed-up administrator operation only where lawful and technically safe. | Retention obligations must be reviewed before production release. |
| Data at rest and portable restore | Candidate: SQLCipher-encrypted DB and protected local key; encrypted, integrity-checked backup envelope with an explicit recovery-key/passphrase flow. Prove offline recovery and loss behavior before architecture lock. | If the recovery secret is lost, encrypted backups may be unrecoverable; show that consequence and test it. |
| “Single final production version” | Freeze the initial feature scope and finish architecture now; maintain a reproducible release pipeline for critical security/data-integrity fixes. | If the owner rejects all future security binaries, record that as accepted residual risk; do not imply ongoing support or silently disable the update path. |

## 3. Functional requirements

Each bullet is normative and carries its stable traceability ID.

### 3.1 Product shell, design and accessibility (UX)

- **UX-001 — Operational shell:** Provide the branded DCMS Pro header, clinic identity, current clinic date, notification center, signed-in user and session/security state.
- **UX-002 — Complete navigation:** Practice: Dashboard, Patients, Appointments, Queue. Clinical: Treatments, Prescriptions. Billing: Invoice, Payments, Inventory, Accounting. Administration: Staff & Users, Backup & Restore, Settings, About. Every item opens a real, permission-aware screen.
- **UX-003 — Navigation behavior:** Sidebar expands/collapses; target about 280 px expanded and 72 px collapsed. Active, hover, focus, disabled and selected states are visible and keyboard accessible.
- **UX-004 — Responsive layout:** Use a consistent spacing/type system and a responsive 12-column grid where appropriate; target 24 px desktop page padding and 68–76 px header. Balanced card placement must not wrap accidentally. Support 1024×768, 1280×720, 1366×768, 1440×900, 1920×1080, 2560×1440 and 3840×2160 at practical coverage.
- **UX-005 — Scaling and overflow:** Verify 100%, 125%, 150%, 175% and 200% Windows scaling where possible. Long pages/forms/modals scroll; tables scroll or adapt rather than clip; no essential control becomes unreachable; no text escapes a card, tab, modal, button or navigation item.
- **UX-006 — Premium design system:** Document palette, typography, spacing, elevation, states, icon sizing/alignment, responsive behavior and reduced-motion behavior. Typography has explicit application/page/section/card/body/metadata/table/label/button/input/helper/error roles with consistent sizes and line heights. Record a coherent palette, initially considering `#0F172A` navy, `#0F766E` teal, `#2563EB` blue, `#F8FAFC` background, `#FFFFFF` surface, `#64748B` secondary text, `#16A34A` success, `#D97706` warning, `#DC2626` danger and `#E2E8F0` border; Phase 2 may refine values after contrast/visual review. Use locally bundled, redistributable fonts with Bengali coverage; no paid icon/font/component dependency. Motion is restrained, performant and honors reduced motion where available.
- **UX-007 — Honest interaction:** No false/fake button, navigation, tab, upload, print, export, search, filter, sort, pagination, date range, modal, save, cancel or delete. If an action is not delivered in the current scope, do not present it as working.
- **UX-008 — Data states:** Every data-driven screen provides loading, populated, meaningful empty and actionable error states. Loading never substitutes fabricated data. Retry exists where safe.
- **UX-009 — Keyboard/accessibility:** Visible focus, logical reading/tab order, reliable Enter/Escape behavior, keyboard shortcuts for global search, navigation, new patient/appointment/visit/prescription/invoice, save/cancel/back/print/lock, and no focus trap or keyboard-only dead end.
- **UX-010 — High-DPI controls:** Icons and button labels are consistently sized and optically centered; text remains readable with long and Bengali content. Dialog dimensions are constrained to the usable viewport and scroll when needed.

### 3.2 Activation, onboarding and practice configuration (SET / ADM)

- **SET-001 — Offline activation gate:** First-launch activation precedes application setup and production access. Verify locally using an Argon2id-derived, salted verifier and constant-time comparison. Do not log, persist, display, test-fixture, document or scatter plaintext activation code. Handle invalid/repeated input professionally and honestly explain the local/offline nature; no network call.
- **SET-002 — Setup wizard:** Collect clinic/dental-care identity, logo, address, contacts, business identity, hours, footer/advice/invoice configuration and defaults; validate each step and provide previous/next navigation.
- **SET-003 — Dentist profiles:** Capture professional name, designation(s), qualification(s)/certification(s), contact information, schedule and signature information where used. Support multiple independent dentist records and multiple designation/qualification rows per dentist.
- **SET-004 — Initial owner and defaults:** Create the first administrative login without a default password; initialize security, auto-lock, backup, printer and foundational application settings. Create baseline roles and permissions.
- **SET-005 — Recoverable, atomic setup:** Back/forward preserves valid entries. Interruption resumes a durable setup draft. Production navigation is unavailable until final validation and one atomic finalization succeeds; failures cannot expose a half-configured clinic.
- **SET-006 — Setup review:** Show a review/confirmation step and explain selected storage, printer/backup and security settings before finalizing; allow safe correction before commit.
- **ADM-001 — Real settings center:** Provide working controls for clinic profile/branding; dentist profiles; users/roles; appointment/patient settings; clinical/prescription templates; invoice/BDT settings; printer/paper profiles; notifications; backup; auto-lock/security; database maintenance; audit; appearance; localization/date/time. Only settings actually supported may appear.
- **ADM-002 — Destructive configuration safeguards:** Separate destructive actions, state consequences, require explicit confirmation and current admin authentication where appropriate, and create a pre-action backup when technically safe. Cancel must leave data unchanged.
- **ADM-003 — About:** Show DCMS Pro product information and only the supplied creator name/email; make no unsupported corporate or professional claims.

### 3.3 Authentication, authorization and records security (SEC)

- **SEC-001 — Password handling:** Unique usernames; modern salted Argon2id password hashing; never store/log/display passwords or put them in configuration. Validate credentials, failed logins and session transitions.
- **SEC-002 — Roles:** Baseline Owner/Admin, Dentist, Receptionist, Assistant/Nurse, Accountant/Finance and Inventory/Store; administrators can configure additional roles and permissions. Staff profile and login account are separate and optional.
- **SEC-003 — Granular permissions:** Independently govern view/create/edit/delete/void/finalize/export/print and destructive operations by module/entity; include patients, demographics/sensitive history, visits, dental chart, prescriptions, appointments, queue, treatments, invoices, payment entry/history/analytics, inventory, accounting, reports, staff/PII/salary, users/roles, backup, restore, settings, audit, search and notifications.
- **SEC-004 — Service-layer enforcement:** Every privileged Rust use case and every result/search query verifies the authenticated session and permission. Financial permission is independent from invoice/payment entry; restricted financial totals, balances, reports and search previews are not returned to unauthorized clients. Hiding UI controls is never the enforcement boundary.
- **SEC-005 — Auto-lock:** Administrator-configurable 5/10/15/30-minute inactivity timeout, lock screen hides protected data, re-authentication required. Warn or preserve drafts before lock; never silently discard unsaved work.
- **SEC-006 — Audit events:** Record timestamp, user, action, entity/type/id, outcome, safe summary and suitable metadata for login/logout/failures, patient and clinical changes, prescriptions, invoice/payment, inventory/accounting, staff/users/roles, backup/restore, destructive and settings/security actions. No secret or unnecessary clinical detail in logs.
- **SEC-007 — Audit protection:** Audit access is permission-gated and user-facing edits/deletes are disabled or tightly restricted. Make ordinary-user tampering evident (append-only application path and, if technically sound, chained integrity hashes); explicitly do not promise forensic protection from a local OS administrator or binary/database owner.
- **SEC-008 — Local data protection:** Protect application data directories using Windows user-scoped storage/ACLs and a vetted at-rest encryption/key protection design. Clearly document the local administrator/malware threat boundary and recovery-key consequences. No plaintext test accounts or development bypasses in release builds.
- **SEC-009 — Safe commands and files:** Parameterize SQL, validate server-side, sanitize app-managed file names/paths, reject path traversal/reparse-point escapes, validate imported/backup content, and never trust frontend state for authorization or values.
- **SEC-010 — Least privilege:** Tauri capability allowlists are narrow, no general frontend SQL command, no broad shell execution, no renderer Node privileges, no remote code or remote origin requirement, and no unnecessary filesystem scope.

### 3.4 Patients, profile and longitudinal clinical history (PAT / CLN)

- **PAT-001 — Patient registration:** Capture configurable unique patient code, full name, age/date of birth appropriately, gender, blood group where appropriate, address, phone, emergency contact, presenting/previous problems, allergies, relevant medical/dental history, notes, preferred language, referral source and registration date. Keep sensitive fields purpose-limited and permissioned.
- **PAT-002 — Patient list:** Newest registrations first by default; indexed/debounced search, sorting and real filters by code, name, phone, address, date range (Today, 7/30/90 days, 1 year, custom), status and useful dentist criteria; paginated/virtualized without a fixed record cap.
- **PAT-003 — One persistent longitudinal profile:** Unlimited visits, appointments, prescriptions, treatment records, invoices, payments, attachments, referrals, timeline events and dental chart history subject to device/database capacity.
- **PAT-004 — Profile overview:** Show visit count, latest visit, upcoming appointment, recent activity, timeline, demographics/contact, medical/dental history/allergies, appointments, visits, treatments, prescriptions, referrals, attachments and dental-chart history.
- **PAT-005 — Financial privacy:** Show invoice total, amount paid, outstanding amount and payment history only to users with the appropriate independent financial permission. Clinical/patient permission alone does not imply financial access.
- **PAT-006 — Context workflows:** From the profile, authorized users can create a visit, appointment, prescription, invoice, payment, referral, attachment and note with patient identity carried safely into the real workflow.
- **PAT-007 — Record retention:** Patient deactivation/soft deletion and corrections retain dependent clinical/financial history and audit trail. No uncontrolled cascade deletes or silent history overwrite.
- **CLN-001 — Distinct visits:** A visit is not an appointment. Persist patient, clinician, timestamp, reason/complaint, examination, assessment/diagnosis, chart findings, procedures/treatments, anesthesia/material details when entered, prescription, advice/follow-up, referral and attachments as applicable.
- **CLN-002 — Editable clinical capture:** C/C, O/E and R/E/advice offer optional configurable choices plus free-form notes. Custom clinician text is never forced into the vocabulary; no diagnosis is mandatory merely because an option exists.
- **CLN-003 — Clinical templates:** Authorized users configure and version visit, complaint, examination, advice and prescription template content. Editing a template never changes previously saved records.
- **CLN-004 — Clinical amendments:** Historical clinical records are preserved. Corrections/addenda require permission, reason and audit; finalized records are not silently replaced.
- **CLN-005 — Clinical timeline:** Chronological timeline reflects actual registration, appointment lifecycle, visit, observation, chart/treatment, prescription, referral, invoice/payment when permitted, attachment, note and follow-up events. Events record timestamp, actor, dentist/source where relevant, and metadata; timeline is filterable and never fabricated.

### 3.5 Dental chart, treatment catalog, referrals and attachments (DENT / TX / FILE)

- **DENT-001 — Dentition and numbering:** Accessible visual and keyboard selection for all 32 adult teeth and 20 primary teeth using standard FDI numbering; visibly differentiate adult and pediatric charts.
- **DENT-002 — Conditions and surfaces:** Support healthy, caries, restored, missing, extracted, root-canal treated, crown, bridge, implant, impacted, fractured, mobility, periodontal involvement and configurable additional conditions. Where applicable support mesial, distal, occlusal/incisal, buccal/facial, lingual/palatal surfaces and one/multiple teeth.
- **DENT-003 — Historical chart:** Store chart observations/conditions against visit/time so later changes do not destroy prior state. Clinicians document findings/treatments; include/print the configured view without chart clipping on smaller layouts.
- **TX-001 — Treatment catalog:** Code/name/category/description/standard fee/duration/tax-or-discount behavior/active state and configurable custom entries. Editing a catalog fee never changes a historical service or invoice snapshot.
- **TX-002 — Treatment record links:** Preserve links to patient, visit, clinician, treatment and invoice where applicable; transactions are atomic and audit controlled historical edits.
- **FILE-001 — Attachments:** A real Windows file picker selects/copies an allowed file into app-managed storage; validate existence/type/size, preserve original source, safely preview supported files, provide a clear external-open path for unsupported types, handle missing files, and delete only with permission and audit. Store opaque safe identifiers/metadata, never trust arbitrary source paths as storage locations.
- **FILE-002 — Referrals:** Record patient, source/recipient, clinician, date, reason/instructions and follow-up; show as a genuine timeline/profile event and preserve history.

### 3.6 Appointments and queue (APT / QUEUE)

- **APT-001 — Appointment data:** Patient, dentist, start/end/duration, reason/type, status, notes, source and create/update metadata; default appointment statuses include scheduled, confirmed, arrived, in queue, in progress, completed, cancelled, rescheduled and no-show.
- **APT-002 — Views and controls:** Real day/week/month and list views with date, dentist, status, patient and appointment-type filters; sorting/date controls change displayed records.
- **APT-003 — Lifecycle:** Create from appointment screen or patient context. Record arrival/completion/no-show/cancel; rescheduling preserves old time/status history; cancellation/no-show are explicit and audited.
- **APT-004 — Scheduling safety:** Use dentist working schedule and configurable duration/appointment rules; detect overlapping assignments and clearly warn or block per clinic policy. Any permitted override is authorized and audited; never silently double-book.
- **QUEUE-001 — Real daily queue:** Show today's arrived/waiting patients, appointment, arrival time, dentist, reason, priority where enabled, state and elapsed/estimated wait.
- **QUEUE-002 — Fast actions:** Authorized queue users can move through arrived, waiting, called, in treatment, completed, skipped and configured states with immediate persisted result, audit and keyboard-friendly workflow; avoid unnecessary dialogs.
- **QUEUE-003 — Queue accuracy:** Derive queue state and waits from persisted timestamps and appointment/arrival transitions. Queue actions remain idempotent/safe under rapid repeated input.

### 3.7 Prescriptions and clinical documents (RX / DOC)

- **RX-001 — Prescription workflow:** Create from a patient context or global section; every prescription has a patient, prescriber, date/time and persisted state.
- **RX-002 — Medicine lines:** Multiple lines with custom/free-text medicine name, form, strength/power, dose, frequency, timing, before/after food, duration, quantity, route, conditional instruction and additional instructions; templates are reusable but never mandatory.
- **RX-003 — Complaint/examination capture:** Offer configurable selectable dental complaints (including pain, G. caries, swelling, gum bleeding, halitosis/bad breath, sensitivity) and observations (including caries/G. caries, configured BDR/BDC terminology, gingivitis, periodontal pocket/periodontitis, pulpitis, impacted teeth, dry socket, attrition/erosion). Keep them optional and editable with free text; no automatic clinical recommendation.
- **RX-004 — Finalization integrity:** Preview/edit before finalization; finalized prescription contents and the rendered document correspond to the persisted record. Amendments require permission and history; no invisible rewrite.
- **DOC-001 — Prescription identity:** Document contains clinic name/logo/address/phone; prescriber name, multiple designations/certifications/qualifications; patient name/gender/age-or-date as configured/code and date. Long identities wrap professionally.
- **DOC-002 — Prescription hierarchy:** Distinct clinical complaint/examination/advice area and multiple medicine lines with readable hierarchy; configured clinic messages/schedule/follow-up/emergency text; substantial blank signing area above a signature line. No default content crowds or pushes the signature unexpectedly.
- **DOC-003 — Paper profiles:** A4, A5, supported thermal/receipt/mini widths and supported custom dimensions; portrait/landscape where useful, appropriate margins, scaling and pagination per document profile. Narrow paper gets a dedicated layout, not blindly scaled A4.
- **DOC-004 — Preview and printer selection:** Real preview before printing; list/choose Windows-installed printers and supported settings/copies; standard Windows drivers, including user-selected Microsoft Print to PDF/equivalent where present. Direct PDF export is separate and local.
- **DOC-005 — Unicode and output integrity:** Bundled redistributable font fallback renders Bengali/English in app preview, PDF and print. Long Bengali/medicine/clinic/qualification text wraps; no clipping, corrupt shaping, table/page overflow, orphaned totals or lost signature area. Test generated PDFs programmatically and visually at paper profiles.
- **DOC-006 — Print error handling:** Printer unavailable/offline, invalid paper/driver, cancellation, file permission and PDF failures produce understandable errors and retry/cancel recovery; never report success before a completed print/export operation.

### 3.8 Invoices, payments and accounting (BILL / PAY / ACC)

- **BILL-001 — Invoice calculation:** Unique configurable invoice numbers; patient/date; catalog or custom line snapshots with description, quantity, unit price, discount, optional tax/adjustment, subtotal, total, paid, due and status. Use integer poisha/decimal-safe math, explicit rounding and validated amounts.
- **BILL-002 — Historical and transactional integrity:** Snapshot descriptions, prices, discounts/tax and patient/document identity as required at issue time; never recompute old invoice line prices from today's catalog. Invoice creation and linked visit/treatment operations commit atomically where appropriate. Duplicate invoice numbers are impossible.
- **BILL-003 — Payment states:** Support unpaid, partial, full, overpayment policy and outstanding balances from actual allocations. A failed DB write cannot show a successful invoice/payment. Corrections use void/reversal/refund entries, not deleting settled history.
- **BILL-004 — Invoice document:** Clinic branding, invoice/date/patient, detailed items, quantity, unit prices, discounts/tax, subtotal, total, paid/due and payment status; no doctor signature unless an explicit setting is added. A4/A5/thermal/mini profiles paginate and keep totals together/readable.
- **PAY-001 — Payment record:** Patient, independently recorded transaction, invoice allocation(s), timestamp, amount, method, reference/transaction ID, receiver and notes. Methods: Cash, Bank, Card, Mobile Wallet; wallet classification includes bKash, Nagad, Rocket, Upay and Others. This is offline classification, not an API integration.
- **PAY-002 — Payment screens and filters:** Today default, 7/30/90 days, 1 year and custom date range; totals/reports reflect actual persisted eligible transactions and user permissions.
- **PAY-003 — Allocation invariant:** Sum of allocations cannot exceed captured payment except an explicitly supported unapplied credit; invoice paid/due is derived consistently. Reversals/refunds remain traceable and balance correctly.
- **ACC-001 — Income/expense ledger:** Configurable categories; patient receipts are not double-counted as separate income; other income and expenses include rent, electricity, internet, staff salary, dental accessories/materials, equipment, maintenance, transportation, office costs, utilities and other clinic-configured categories, with timestamp, amount, category, user, payee/source, notes and audit.
- **ACC-002 — Financial reports:** Daily/weekly/monthly/quarterly/yearly/custom totals, income-versus-expense, receivables, payment method, expense category and inventory purchases; use consistent BDT dates/rounding and actual transactions.
- **ACC-003 — Financial isolation:** Require explicit financial/report permissions at business-service and query level for every total, balance, detail, export, print and search preview. Invoice/payment entry permission may exist without clinic-wide revenue analytics.
- **ACC-004 — Report output:** Authorized export/print uses real filtered data, records selection/time/user in appropriate audit, and handles empty/error states honestly.

### 3.9 Inventory, staff and administration (STOCK / STAFF / ADM)

- **STOCK-001 — Inventory metadata:** Track dental accessories, consumables, materials, medicines and clinic supplies. Capture item name, SKU/code, category, supplier/purchase source, unit, purchase/usage cost, opening/on-hand quantity, minimum threshold, batch/lot, expiry/purchase dates, supplier contact, notes and audit metadata.
- **STOCK-002 — Stock movements:** Persist stock-in, consumption/stock-out and reasoned manual adjustments as append-only transactions; derive on-hand quantity; supplier management, inventory reports and purchases.
- **STOCK-003 — Stock safety:** Prevent negative on-hand stock by default; configurable exception only with explicit authority/policy. Every adjustment is audited; low-stock and expiring-soon notifications are generated from current persisted state and deduplicated.
- **STOCK-004 — Expiry and report accuracy:** Filter/report by supplier, category, low stock, expiry and date; expiry warnings use configured clinic thresholds and real batch quantities.
- **STAFF-001 — Staff profiles:** Purpose-appropriate name, section/role, DOB/age where configured, address, blood group where justified, ID, photo, contacts, salary, employment status, joining date and notes. Sensitive PII/salary access is separately permissioned and minimized.
- **STAFF-002 — Staff/account separation:** A staff member may have no login. Login username is unique; account enable/disable and staff employment status are distinct and audited.

### 3.10 Dashboard, search, notifications, backup and operations (DASH / SEARCH / NOTIFY / BACKUP)

- **DASH-001 — Operational dashboard:** Data-driven today patients/appointments, waiting queue, completed visits, pending/missed appointments, recent patients/activity, upcoming appointments, inventory low/expiry and actionable notices. Empty/no-data states are useful; no production hardcoded demo values.
- **DASH-002 — Permission-aware finance:** Today revenue/payments/receivables and financial chart widgets appear only when separately authorized; clinical/reception roles see only relevant permitted information.
- **DASH-003 — Real widgets:** Chart totals/series derive from database, support permission-safe configurable widgets where appropriate, and provide loading, failure and empty states with retry when meaningful.
- **SEARCH-001 — Global search:** Search patients by name/code/phone/address/configured identifiers; appointments by patient/date/dentist/status; prescriptions by patient/date/dentist/medicine; invoices by number/patient/date/status; payments by patient/reference/date/method; inventory by item/SKU/supplier/batch/expiry; staff/users by name/role/status; and permitted clinical records. Search is debounced, indexed, keyboard-shortcut accessible, paginated and useful.
- **SEARCH-002 — Search authorization:** Apply the same permission checks and field filtering to query, counts, snippets and result previews; never leak restricted clinical, salary or financial values through global search.
- **NOTIFY-001 — Real notification center:** Persist read/unread/history for actual upcoming/missed appointments, low/expiring stock, authorized outstanding balances, backup results/failures, security events and actionable tasks. Permission-filter, deduplicate and avoid spam.
- **BACKUP-001 — Configurable local backup:** Admin selects a folder with a real Windows folder picker; manual and 7/15/30-day schedule, safe timestamped filenames, retention policy, progress/cancel behavior, and actual status notification. No hardcoded production path.
- **BACKUP-002 — Complete, consistent contents:** Include a transactionally consistent database and required attachments/configuration/recovery metadata; exclude caches, temporary files, logs and unrelated secrets. Integrity hashes and schema/version manifest.
- **BACKUP-003 — Safe restore:** Select and validate one self-contained backup at a time; verify checksum, encryption/authenticity, database integrity and supported schema before restore; create and verify a pre-restore backup first; stage restore and atomically switch only after success; preserve current database on failure.
- **BACKUP-004 — Recovery UX:** Show progress without freezing UI; handle locked/inaccessible folders, missing media, disk full, wrong recovery secret, corrupt/partial/incompatible backups and cancellation with a clear recoverable explanation. Keep recovery path and audit event.
- **OPS-001 — Error handling:** Return safe structured application errors, field-level validation, retry where useful and local technical logs with redaction. No raw stack trace for normal users, silent swallow, false success or logged secrets/health narrative.
- **OPS-002 — Performance:** Debounce search; paginate/virtualize large lists/timelines/attachments; run DB, backup, restore and PDF work off UI thread; keep UI responsive and use realistic test data.
- **OPS-003 — Data lifecycle:** Graceful application restart/close, single-instance or serialized DB safety, clean migrations, durability after restart and explicit data-preserving uninstall behavior.
- **OPS-004 — Application icon:** Original, optically centered transparent source artwork combining a custom dental identity with DCMS Pro mark; build and inspect appropriate ICO sizes at taskbar/shortcut/Start Menu/installer/titlebar; no emoji/stock icon/cropped or white-corner artifact.

### 3.11 Database, documents, deployment and release gates (DB / DOC / REL)

- **DB-001 — Relational model:** Normalize clinic, dentists/designations/qualifications/schedules, staff/users/roles/permissions, patients/contacts/history/allergies, visits/observations/chart/tooth conditions, treatment catalog/records, appointments/status history/queue, prescriptions/items/templates, medicines, referrals, attachments, invoices/items, payments/allocations/methods, inventory/suppliers/batches/stock transactions, income/expense/categories, notifications, audit, backup metadata, printer profiles and settings as needed.
- **DB-002 — Integrity:** Foreign keys on every connection; explicit uniqueness/check constraints and indexes for actual filters/search; transactions for multi-step operations; intentional cascade/restrict/soft-delete policy; no orphaned clinically/financially important records.
- **DB-003 — Migrations:** Versioned, ordered migration files; clean install and supported in-place migration tests; schema compatibility checks for restore; migration failure preserves recoverable database and reports failure.
- **DB-004 — Unlimited records/performance:** No artificial patient/history max. Store query/page cursors safely and test large synthetic data without shipping it. SQLite/database/file capacity remains the technical bound.
- **DOC-007 — Shared rendering:** Reusable local document model/template/rendering with separately configured prescription and invoice layout, deterministic paper profile, preview, print settings, pagination, Unicode fonts and testable generated artifact.
- **REL-001 — CI quality gates:** GitHub Actions installs locked dependencies; lint/type/static checks; unit, DB, authorization, integration, document and UI/E2E tests; Windows production build; release validation. Any required failing test/build blocks artifact publication.
- **REL-002 — Windows artifact:** Generate the production installer/executable on a Windows GitHub runner; production mode only, version/product metadata and icon verified, no dev server/debug menu/maps/mock data/test accounts/bypass, artifacts retained and SHA-256 checksummed.
- **REL-003 — Install/uninstall:** Start Menu entry, appropriate configurable/standard desktop shortcut, install/reinstall/upgrade behavior, Windows registration, working uninstaller and documented user-data retention (ordinary uninstall preserves clinic data unless an explicit separately confirmed deletion is chosen).
- **REL-004 — Clean-machine acceptance:** On isolated supported Windows, install without developer tools/source tree, activate, finish setup and exercise a representative patient/visit/appointment/prescription/invoice/payment/inventory/expense/backup/restart/restore/PDF/print workflow; verify uninstall/data policy.
- **REL-005 — Dependency/license audit:** Audit direct and transitive packages, runtime, fonts, icon sources and generated assets for commercial redistribution, obligations and vulnerabilities. Include required third-party notices/license text; avoid paid/closed/AGPL/GPL dependencies unless legal review approves a compatible use.
- **REL-006 — Release review:** Requirements-to-implementation and acceptance matrix review twice (before RC and immediately before final build); inspect every screen/route/action/permission, database relation, document, installer, workflow, logs, dependencies, secrets, external calls and TODO/development-only code. No release-blocking/high-severity defect or failing release gate.
- **REL-007 — Release publication:** Publish a GitHub Release with installer, release notes, checksums and relevant notices only from the final tested source/tag when permissions allow. If verified platform/permission limitations prevent release publishing, retain only intentional final artifacts/support files in `dist`; do not claim an unpublished release exists.
- **REL-008 — Human-controlled PRs:** Work stays on the Arena session branch. Create/inspect coherent phase PRs when appropriate; the owner decides whether to merge. The agent must never merge.

## 4. Data integrity and calculation invariants

These are cross-cutting pass/fail invariants, not UI validation suggestions:

1. Patient code, username, role/permission identifiers as applicable, invoice number and per-clinic document sequence are unique under concurrent requests.
2. A visit, linked treatments, invoice, payment allocation and required audit events either commit atomically or do not appear as success.
3. Prescription preview/print/PDF consumes the persisted prescription snapshot; it must not create a document from uncommitted or stale frontend data.
4. An invoice's historical line amount and description remain unchanged by catalog edits; old BDT balances are derived from that invoice snapshot and posted allocations/reversals.
5. A payment is successful only after transaction commit. Allocation totals, refunds/reversals, due balance and reports reconcile exactly in integer poisha.
6. Stock-on-hand equals opening stock plus persisted stock-in/adjustment minus persisted usage/outflow, under the configured negative-stock policy. Failed movement never changes the displayed balance.
7. Accounting reports aggregate each underlying event once; patient payments are not double-counted as a second income entry.
8. Audit events record the committed actor/action/outcome; failed business transaction does not leave a false success event (security failure attempts may be separately logged).
9. Backups capture a consistent database/file snapshot. Restore never replaces a healthy live database with an unvalidated/partial candidate.
10. Authorization is checked again in service/repository use cases, including IDs provided by the UI. A hidden control, modified route, forged IPC payload or saved frontend state cannot grant a capability.

## 5. Quality and completion definition

A requirement is complete only when implementation, integration, persistence/data integrity, authorization, success and negative/error paths, visible UX/accessibility, relevant regression, documentation and verifiable acceptance evidence pass. A compile-only result is not completion. The phase plan defines per-phase gates; the traceability matrix records test and screen linkage. Unavailable Windows hardware/environment evidence is explicitly marked unavailable, not inferred.
