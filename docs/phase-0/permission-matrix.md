# DCMS Pro — Proposed Default Permission Matrix

**Status:** Phase 0 security baseline. Phase 1 must turn every cell into explicit permission keys and test direct service-layer denials. The owner can configure custom roles; defaults are least-privilege starting points, not immutable access grants.

## Legend and policy

- **A** = allow the indicated ordinary actions by default. **L** = limited/scope-limited allow described below. **—** = deny by default.
- Permission keys distinguish `view`, `create`, `edit`, `finalize/void`, `export/print`, `manage`, and `delete/destructive`; one “module access” bit is not sufficient.
- Role grants are enforced in Rust use cases and query projections. UI visibility is only a convenience. A user with several roles receives the union of explicitly granted capabilities; there is no implicit override/deny ambiguity. Owner/Admin permission edits are audited.
- Every role can be further restricted by clinic policy (including dentist-only patient assignment where configured). No staff/financial/clinical data is included in a search snippet, dashboard aggregate, notification, export or IPC response unless the corresponding permission allows it.
- `L` on invoice/payment entry means the employee can access only the data necessary to create/reconcile that transaction (e.g. the selected invoice amount and amount due). It does **not** imply historical balances across the clinic, revenue analytics, accounting reports or unrestricted patient financial history.
- Sensitive data is independently permissioned: patient clinical history, patient financial history, staff photo/ID/address, salary, audit, and role/security administration. Owner/Admin's broad grant is a default; it remains subject to authenticated session, re-authentication for sensitive actions and audit.

## Capability-family defaults

| Capability family | Owner/Admin | Dentist | Receptionist | Assistant/Nurse | Accountant/Finance | Inventory/Store | Custom role |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Sign in / own session / lock / change own password | A | A | A | A | A | A | A |
| Patient identity/contact view and registration | A | A | A | L | L | — | — |
| Patient demographics/contact edit | A | L | A | L | — | — | — |
| Patient sensitive medical/dental history and allergies | A | A | L | A | — | — | — |
| Patient delete/deactivate/merge or destructive purge | A* | — | — | — | — | — | — |
| Clinical visit, diagnosis/observation and timeline read | A | A | — | L | — | — | — |
| Clinical note/visit create or edit draft | A | A | — | L | — | — | — |
| Clinical finalized-record amendment | A* | A* (own/assigned, reason required) | — | — | — | — | — |
| Dental chart read/update | A | A | — | L | — | — | — |
| Prescription create/edit draft/finalize | A | A | — | L (no prescriber/finalize unless separately granted) | — | — | — |
| Prescription history/export/print | A | A | — | L | — | — | — |
| Appointment view/create/edit/reschedule/cancel | A | A | A | L | — | — | — |
| Queue state/actions | A | A | A | A | — | — | — |
| Treatment catalog view | A | A | L | A | L | — | — |
| Treatment catalog price/administration | A | — | — | — | L | — | — |
| Invoice create/draft/finalize | A | L (if explicitly granted) | A | — | A | — | — |
| Invoice current transaction view (selected invoice only) | A | L | L | — | A | — | — |
| Patient financial history/balance view | A | — | L (current transaction only) | — | A | — | — |
| Payment capture / allocation | A | L (if explicitly granted) | L | — | A | — | — |
| Payment history, void/refund/reversal | A* | — | — | — | A* | — | — |
| Clinic revenue, receivables and payment analytics | A | — | — | — | A | — | — |
| Accounting categories, income and expense ledger | A | — | — | — | A | — | — |
| Financial reports/export | A | — | — | — | A | — | — |
| Inventory items/suppliers/stock levels | A | L (read only if clinically useful) | — | L (read only if needed) | L | A | — |
| Stock-in/out/adjustment and supplier management | A | — | — | L (consumption only if granted) | L (purchases/costs as granted) | A | — |
| Inventory cost/purchase report | A | — | — | — | A | L | — |
| Staff directory (non-sensitive fields) | A | L (assigned team only) | L | L | L | — | — |
| Staff salary/ID/address/photo and employment admin | A* | — | — | — | L (salary only if granted) | — | — |
| User/role/permission administration | A* | — | — | — | — | — | — |
| Backup configuration/manual backup | A* | — | — | — | — | — | — |
| Restore/import/delete-all/business destruction | A** | — | — | — | — | — | — |
| Clinic/security/system settings | A* | — | — | — | L (finance settings only) | L (inventory settings only) | — |
| Global search | A | A | A | A | A | A | — |
| Search clinical result details | A | A | — | L | — | — | — |
| Search invoice/payment/financial result details | A | — | L (transaction-only) | — | A | — | — |
| Search staff salary/private records | A* | — | — | — | L (explicit salary permission) | — | — |
| Notification history | A | L | L | L | L | L | — |
| Audit-log read/export | A* | — | — | — | L (finance audit subset if granted) | — | — |
| Audit-log edit/delete | — (not exposed in ordinary UI) | — | — | — | — | — | — |

`*` Sensitive action requires its specific permission; re-authentication and audit where configured.

`**` Restore and irreversible destruction are separate permissions, explicit typed confirmation, pre-action backup where safe, and current Owner/Admin re-authentication. A separate owner-level policy can disable deletion entirely.

## Permission key design (candidate)

Use stable namespaced keys, not role names in business logic. Examples:

```text
patients.identity.view         patients.identity.create         patients.identity.edit
patients.history.view          patients.history.edit            patients.delete
clinical.visits.view           clinical.visits.create            clinical.visits.edit_draft
clinical.visits.amend          clinical.chart.view               clinical.chart.edit
clinical.prescriptions.create  clinical.prescriptions.finalize    clinical.prescriptions.amend
appointments.view              appointments.manage                queue.manage
catalog.treatments.view        catalog.treatments.manage          billing.invoice.create
billing.invoice.view_context   billing.invoice.history.view       billing.invoice.void
billing.payment.create         billing.payment.history.view       billing.payment.reverse
finance.analytics.view         finance.accounting.manage           finance.report.export
inventory.view                 inventory.stock.manage              inventory.cost.view
staff.directory.view           staff.sensitive.view                staff.salary.view
admin.users.manage             admin.roles.manage                  admin.settings.manage
admin.backup.create            admin.backup.restore                admin.data.destroy
admin.audit.view               files.attach                         files.delete
```

The final permission inventory must also separately cover printing/export, search projections, notification detail, role assignment, data import, device/global shortcuts if privileged, backup destination access, and report-specific data fields. Names are illustrative until the Phase 1 ADR/security model is approved.

## Required authorization negative tests

1. A receptionist with invoice/payment capture but no `finance.analytics.view` cannot obtain clinic revenue, reports, cross-patient outstanding totals, payment history, accounting exports or financial dashboard values by changing route, forged IPC DTO, cached UI state, search term, identifier or pagination.
2. A dentist can create/finalize clinical prescriptions but cannot query accounting or payment details unless explicitly granted a separate permission.
3. An accountant can reconcile invoice/payment fields and reports without receiving clinical notes, allergies, dental chart, prescription content or full medical timeline.
4. An assistant can only create/edit the clinical drafts/consumption fields explicitly granted; cannot finalize a prescription or alter a clinical history through a different endpoint.
5. Inventory/store user cannot reach patient identity/history or finance reports through supplier, stock or global-search results.
6. A custom role with no grants is denied by default. Removing a role grant invalidates active-session privileges immediately or at a documented safe boundary; stale client caches must not retain access.
7. Every negative operation returns a safe denial, creates the appropriate security audit event without disclosing protected data, and leaves database state unchanged.
8. Role/permission edits, restore, irreversible deletion, audit export, invoice void and payment reversal enforce re-authentication/authorization at the service boundary and have cancel/no-change tests.
