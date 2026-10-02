# Security, RBAC, activation and session design

**Status:** Proposed security model. No authentication/RBAC code or security tests exist. The controls below are design obligations and remain subject to Windows DPAPI/SQLCipher/recovery proof; they are not a security certification.

## 1. Security objectives and boundaries

### Protect

- Patient identities/contact details, medical/dental history, visits, chart findings, prescriptions, referrals and encrypted attachments.
- Invoices, payment allocations/reversals, BDT balances/reports, purchase costs, staff salary/ID/address and audit history.
- Password verifiers, activation verifier, database/attachment keys, recovery private identity, temporary print/PDF content and backup manifests.
- Correctness of clinical/financial/stock state transitions, safe recovery and authorization across screens, search, totals, events, documents and exports.

### Adversaries considered

- A normal in-app user attempting an operation beyond their role by changing a route, forging an IPC DTO, reusing stale UI state, guessing IDs, altering filters/cursors or racing an action.
- An unauthorized person copying a database, attachment directory or removable backup while not logged into the designated Windows profile and lacking the portable recovery identity.
- Accidental disk-full, corruption, process crash, wrong key, printer failure, malformed import/attachment or interrupted migration/restore.
- A remote page/script attempting to access app commands or exfiltrate data through the renderer.

### Explicitly not prevented

A determined local administrator, malware running as the designated Windows user, a debugger/patch to the offline executable, kernel compromise, authorized-user screenshots/exports, a recovery private key copied by its owner, or physical observation of a printed chart can defeat controls. DPAPI CurrentUser ties the local wrapper to the profile; it is not protection against malware in that same profile. App RBAC is not a substitute for Windows account security/BitLocker. No DRM, forensic immutability, FIPS certification or legal/regulatory compliance is claimed.

## 2. Data classification and projection policy

| Class | Examples | Default handling |
|---|---|---|
| Public product metadata | Product name, supplied creator details, version | May appear in About and installer; do not invent a company/publisher. |
| Operational identity | Patient code/name/phone, appointment time, staff directory | Only the minimum projection for the use case; query-level permission; no unrestricted search snippet. |
| Clinical sensitive | Allergy, history, chart, visit/diagnosis/observation, prescription/referral | Separate `patients.history.*` / clinical keys; protect in print/search/notifications; no generic logs. |
| Financial sensitive | Invoice amounts, due/payment history, revenue, cost, accounting | Entry/context permissions are separate from historical/clinic-wide analytics and exports. Store money as integer poisha. |
| Staff sensitive | Address, ID, photo, salary | Directory, PII and salary are separate keys; salary never piggybacks on staff-list permission. |
| Secret/key material | Password PHC, activation verifier, `K_db`, attachment key, recovery identity | Never sent to renderer or logs. `K_db` DPAPI CurrentUser; portable recovery key user-held; authenticated encryption. |
| Diagnostic | Correlation IDs, operation kind, safe OS error class | Local ACL-protected, bounded and redacted; no patient names, notes, amounts, paths with identifiers, code/verifier or stack trace in normal UI. |

## 3. Permission model

### Enforcement model

- Permission keys are stable, namespaced strings seeded by migrations; role names never appear in domain authorization conditions.
- Effective permission set is the **union of explicit grants** across the authenticated user's assigned roles. No implicit Owner override and no deny-role precedence. Custom roles begin with zero grants. Remove/disable a role and bump `authz_epoch`; backend checks the new state before the next command and clears the active user's frontend cache.
- Every command resolves the actor from the server-side session, validates the account remains enabled, verifies the exact capability and any row/clinic scope, then queries only allowed fields. No caller-supplied `user_id`, `role_id`, `is_admin` or permission bit is authority.
- Permission applies to every representation: list rows, detail fields, totals/counts, chart/dashboard aggregates, filter facets, search result snippets, notification previews, PDF/print, attachment open/export and audit. A UI guard is only a convenience.
- Dentist assignment restriction, if enabled, is an explicit server-side subject/resource scope (e.g. assigned dentist/patient care-team relationship), not a client filter. It must be documented and negative-tested. The initial default is clinic-wide within the clinician's permitted data family; owner confirmation is required before narrowing scope.
- Operation-specific re-authentication is additive to permission grants. Re-auth validates the current user's password within a short window and is invalidated by lock, logout or account/role change.

### Stable permission-key catalogue (candidate)

#### Session and patients

```text
session.sign_in                 session.lock                    session.logout
session.password.change_own     session.reauthenticate
patients.identity.view          patients.identity.create        patients.identity.edit
patients.identity.deactivate    patients.identity.merge
patients.contacts.view          patients.contacts.edit
patients.history.view           patients.history.edit           patients.history.amend
patients.financial_context.view
```

#### Clinical and scheduling

```text
clinical.visits.view            clinical.visits.create_draft    clinical.visits.edit_draft
clinical.visits.finalize        clinical.visits.amend
clinical.chart.view             clinical.chart.record_finding    clinical.chart.amend
clinical.treatments.view        clinical.treatments.record      clinical.treatments.amend
clinical.prescriptions.view     clinical.prescriptions.create   clinical.prescriptions.edit_draft
clinical.prescriptions.finalize clinical.prescriptions.amend     clinical.prescriptions.print
clinical.referrals.view         clinical.referrals.create       clinical.referrals.edit
appointments.view               appointments.create              appointments.manage
appointments.reschedule         appointments.cancel              appointments.override_overlap
queue.view                      queue.manage                     catalog.treatments.view
catalog.treatments.manage       clinical.templates.manage
```

#### Billing, money, stock and staff

```text
billing.invoice.create          billing.invoice.view_current     billing.invoice.history.view
billing.invoice.issue           billing.invoice.void              billing.invoice.print
billing.payment.capture         billing.payment.allocate          billing.payment.history.view
billing.payment.reverse         billing.payment.refund
finance.analytics.view          finance.accounting.view            finance.accounting.manage
finance.report.export           finance.inventory_cost.view
inventory.items.view            inventory.stock.manage            inventory.suppliers.manage
inventory.cost.view             inventory.report.view
staff.directory.view            staff.sensitive.view               staff.salary.view
```

#### Documents, files, search, administration and audit

```text
documents.preview               documents.print                   documents.pdf.export
files.attach                    files.preview                     files.export
files.delete
search.global                   search.clinical                   search.financial
search.staff_sensitive
notifications.view              notifications.mark_read
admin.users.manage              admin.roles.manage                admin.settings.manage
admin.audit.view                admin.audit.export
admin.backup.create             admin.backup.configure            admin.backup.restore
admin.data.destroy              admin.maintenance
```

A permission seed is not a grant. If an endpoint needs a new operation or protected field, add a specific key, default-role decision, audit rule and direct service test before surfacing it.

### Role defaults (design baseline; owner configurable)

`A` = ordinary default grant, `L` = narrow transaction/scope grant, `—` = deny. These groups summarize the Phase 0 matrix; the exact stable keys above are the authorization unit.

| Data/action family | Owner/Admin | Dentist | Receptionist | Assistant/Nurse | Accountant/Finance | Inventory/Store | Custom role |
|---|---|---|---|---|---|---|---|
| Own session/lock/change password | A | A | A | A | A | A | A |
| Patient identity and contacts | A | A | A | L | L | — | — |
| Demographics edit | A | L | A | L | — | — | — |
| Medical/dental history/allergies | A | A | L (only configured safe/current context) | A | — | — | — |
| Visit/chart/treatment history | A | A | — | L | — | — | — |
| Finalized clinical amendment | A* | A* (authorized scope) | — | — | — | — | — |
| Prescription draft/finalize/print | A | A | — | L (no finalization by default) | — | — | — |
| Appointment/queue | A | A | A | L/A (queue) | — | — | — |
| Treatment catalog | A | A view | L view | A view | L manage prices | — | — |
| Invoice creation/current transaction | A | L only if granted | A/L | — | A | — | — |
| Financial history/analytics/reports | A* | — | — (selected current transaction only) | — | A | — | — |
| Payment capture/history/reversal | A* | L capture only if granted | L capture only | — | A* | — | — |
| Inventory movement/supplier/cost | A | L view | — | L consumption if granted | L cost/purchase | A (movement) | — |
| Staff directory/PII/salary | A* | L assigned team | L basic directory | L basic directory | L salary only if granted | — | — |
| Users/roles/settings | A* | — | — | — | L finance settings only | L inventory settings only | — |
| Backup/restore/data destruction | A** | — | — | — | — | — | — |
| Audit view/export/edit | A* / scoped | — | — | — | L finance subset if granted | — | — / never edit |
| Search/notifications | A | A with clinical scope | A with transaction-safe scope | L | A financial scope | A inventory scope | explicit grants only |

`*` Requires the exact sensitive permission, recent re-auth where configured and audit. `**` Restore and irreversible destruction are separate permissions, strong confirmation, pre-action backup when safe and Owner/Admin re-authentication. Owner/Admin does not bypass session or audit controls. “A” is a proposed default, not immutable access; the owner may reduce it.

### Sensitive projection matrix

- **Reception transaction:** current selected invoice/due and the specific payment action needed. No cross-patient outstanding total, patient-wide history, revenue widget, historical exports or analytics without distinct keys.
- **Accountant:** finance and payment data without clinical notes, allergies, dental chart or prescription body. Patient identity is minimized to the fields needed to reconcile.
- **Dentist:** authorized assigned/clinic patient identity and clinical detail; no clinic-wide revenue/payroll unless separately granted.
- **Inventory:** supplier/item/quantity/cost according to distinct `inventory.cost.*` permission; no patient identity/history through global search.
- **Assistant:** only the explicitly granted clinical draft/read/consumption fields; no prescriber/finalize permission by implication.
- **Notification:** persist event type/recipient/read-state and opaque resource reference; resolve title/body only after current permission check. Do not store protected snippet as a convenience cache.

## 4. Credentials, activation and password verification

### User authentication

- Usernames are unique after documented normalization; passwords are never stored, transmitted outside the local IPC boundary, logged or returned.
- Use the RustCrypto Argon2id implementation behind a maintained password-hashing API with PHC strings. Initial minimum candidate: **m=19 MiB, t=2, p=1** (OWASP Password Storage Cheat Sheet); tune using named minimum Windows hardware and version parameters stored in the PHC string. No custom hash or static pepper in source.
- Password changes/reset require an authenticated, permission-checked workflow. No default account/password or hidden reset password. Owner recovery is separate from data key recovery; a Windows DPAPI profile can still unlock the encrypted database after a permitted password reset.
- Verify in constant time using the selected library's supported API. Throttle repeated failures with bounded exponential delay and audit only account-safe outcome; avoid a permanent lockout that makes local recovery impossible. Do not store plaintext password in retry state.
- Never ship a test user, password, bypass feature or production password literal. Tests generate ephemeral salts/passwords at runtime and assert no credential content in logs.

### Offline activation

- Activation occurs locally before setup/database access. The production verifier is generated outside source control from the owner-provided code using a random salt and Argon2id PHC record; the raw code is **never** placed in source, docs, tests, sample data, CLI output or logs.
- Candidate build interface: a protected release-only build input containing the salted verifier (not the plaintext code), injected from an approved CI secret; no verifier means release build fails closed. Unit tests create an ephemeral verifier at runtime. No development bypass is compiled into production.
- At runtime, accept the code into a zeroized input buffer, verify against the embedded/build verifier using Argon2id's supported verification API, rate-limit failures, and discard input. Persist only an activation marker/version/time protected by DPAPI CurrentUser, never the entered code or verifier in an ordinary settings table.
- This is an offline first-run gate, not strong DRM: a determined binary analyst can extract/replace verifier code or patch the gate. No server, phone-home, hidden network call or unbreakable-licensing claim.
- Reinstallation/data-preservation and reset behavior must be tested; user data and activation state remain on ordinary uninstall unless the owner explicitly selects the separate destructive purge workflow.

## 5. Session, lock and re-authentication lifecycle

- One active application user session per process. Process restart requires login; “remember me” and persistent bearer tokens are not in scope.
- Backend session contains user ID, session generation, authentication time, last in-app activity, permission epoch and disabled/revoked state. Frontend receives a minimal user/permission projection for display only.
- Verify account enablement and current role/version before each protected use case. A role/user change increments `authz_epoch`; affected commands deny until refreshed. Do not trust a cached permission list for service authorization.
- Configurable 5/10/15/30-minute inactivity lock (Phase 0 default choices); manual lock and Windows workstation-lock/suspend lock immediately. On resume, require password/re-auth. If the app is inactive/backgrounded, the timer may continue and must not reset from unrelated OS input unless policy is explicitly changed.
- Before lock, warn/preserve user-authored drafts via an explicit save path; never silently lose clinical work. Hide sensitive windows, close preview/print jobs, cancel fetches where possible, clear renderer query cache/forms and return the backend to `Locked`. Do not imply memory erasure of OS/browser copies.
- Re-authentication is required before restore, hard purge, role/security change, invoice void, payment reversal/refund, audit export, private-data export and key/recovery identity rotation according to exact key policy. Re-auth has a short configurable server-side validity window, invalidated by lock/logout/account/role change.

## 6. Audit, logs and security events

- Audit significant success/failure events with server-derived UTC, actor, event key, entity type/opaque ID, outcome, safe reason, correlation and operation IDs. Use a canonical serialization and a SHA-256 hash chain to detect accidental alteration; state explicitly that a local admin can rewrite the database and recompute an unkeyed chain.
- Append audit rows in the same business transaction as committed patient/clinical/financial/stock/user/role/settings transitions. Authentication-denial events may be recorded separately; they must not include submitted password/code or full clinical/financial request body.
- Audit read/export has separate permissions. Ordinary product UI exposes no update/delete path; cleanup/retention policy must not erase the only legal/business record without owner review.
- Diagnostic logs use safe event codes and correlation IDs. Exclude activation code/verifier, password/PHC, keys/recovery identity, clinical note/body, patient contact, payment reference, full export path and raw DTO. Disable browser console/DevTools in release; no remote crash/analytics collector.

## 7. Local OS and application hardening

- Keep app data under designated Windows user's `%LOCALAPPDATA%`, with restrictive ACLs. Use DPAPI CurrentUser for `K_db`, no LocalMachine scope. A local administrator or same-user process remains in scope as a residual threat.
- SQL parameterization for every user-derived value; strict DTO length/range validation; no SQLite extension loading; foreign keys on; migration allowlist; no arbitrary JSON settings or SQL.
- Managed files use opaque names, canonical-root checks, reparse-point/symlink defense, allowlisted MIME/extension/magic/size, encrypted bytes and per-object authenticated data. Decrypted external-open files are explicit user exports; clean temporary plaintext at best effort and audit. Do not open arbitrary user paths without a picker.
- Tauri capabilities: minimum main-window commands and native plugin permissions; separate print window, no remote origins, no shell/fs/http/process plugin capability, local-only navigation and restrictive CSP. Rust service authorization remains mandatory even if the capability is misconfigured.
- Windows updates/runtime: WebView2 security servicing is a dependency lifecycle; keep an update-support policy and regression process. No product network updater or hidden application fetch.
- Reproducible lockfiles, Windows CI, dependency review, SBOM/license report, security advisories and release bundle secret scan. Do not put owner signing or recovery secrets in Git/chat.

## 8. Required negative-security tests

1. No session/locked/disabled account: every data command denies; no row/field/count leaks.
2. Receptionist with invoice/payment entry cannot retrieve financial analytics, clinic-wide balance, other-patient payment history, export, dashboard or search snippets through forged IDs/routes/cursors/notifications.
3. Dentist cannot retrieve payment/accounting/salary fields; accountant cannot retrieve clinical notes/chart/allergies/prescription content; store user cannot retrieve patients through item/supplier/search.
4. Assistant cannot finalize/print a prescription or amend finalized history absent explicit capability; dentist cannot finalize another restricted scope if care-team policy enabled.
5. Custom role with zero grants denies all protected commands. Role removal invalidates backend authority and cached UI on the next command, including an in-progress export/print lease.
6. Re-auth-required operation with stale timestamp/wrong user fails and leaves state unchanged. Cancelled confirmation makes no DB or filesystem change.
7. Cross-patient/resource-ID forgery, SQL-injection-shaped names/search filters, duplicate JSON keys, malformed enum, path traversal/reparse point and oversized attachment reject safely.
8. Password/activation tests: wrong/correct, repeated attempts, no persistence/logging, no plaintext in tracked files/release bundle; verification and build secret are absent from normal CI logs.
9. SQLCipher wrong key/corrupt DB/file key/data tamper reject; same Windows profile works; other profile cannot DPAPI-unprotect the local key; portable backup only works with correct exported recovery identity.
10. Print/PDF/search/notification/dashboard/document DTO tests assert permission on protected content independently of direct record query.

## References

- Phase 0 permission baseline: [`../phase-0/permission-matrix.md`](../phase-0/permission-matrix.md)
- Phase 0 security requirements/threats: [`../phase-0/requirements-spec.md`](../phase-0/requirements-spec.md), [`../phase-0/risk-register.md`](../phase-0/risk-register.md)
- OWASP Password Storage Cheat Sheet: <https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html>
- Argon2id specification: <https://www.rfc-editor.org/rfc/rfc9106.html>
- Microsoft DPAPI CurrentUser vs LocalMachine: <https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata>
- Tauri capabilities/security: <https://v2.tauri.app/security/capabilities/>
- [IPC/state contract](ipc-and-state-contract.md), [ADR-0002](adr/ADR-0002-data-encryption-and-keys.md)
