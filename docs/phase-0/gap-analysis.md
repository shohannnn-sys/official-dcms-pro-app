# DCMS Pro — Requirements Gap and Ambiguity Review

**Status:** Phase 0 discovery; resolved defaults are in `requirements-spec.md`. Items marked “owner gate” do not block this phase; they must be confirmed or explicitly accepted at the named design/release gate. No omitted item is silently treated as delivered.

## 1. Explicit requirement families traced

The submitted directive is represented by 109 uniquely identified normative requirements in `requirements-spec.md`, with one traceability row and one planned positive/negative/authorization/safeguard/visual acceptance row per ID. Coverage includes product/offline constraints, desktop architecture, setup and activation, clinic/dentist profiles, shell/design/accessibility, patients and longitudinal timelines, visits/templates/chart/attachments/referrals/treatments, appointments/queue, prescriptions/documents/printers/PDF, invoices/payments/accounting, inventory, staff/RBAC/auto-lock, backup/restore, settings/search/notifications/audit, data integrity/performance, Windows installer/CI/releases, test/stress/clean-machine/release audit, documentation, PR/manual-merge and continuation memory.

The following are not product requirements that may be quietly inferred: cloud features, WAN/LAN multi-PC support, direct Bluetooth/USB printer protocols, remote medical advice, online payment processing, automated diagnoses, Bengali translation of all UI chrome, regulatory certification, future features, or fictitious company details.

## 2. Implicit requirements needed for a safe implementation

These requirements are necessary to make the requested workflows reliable. They have been incorporated as constraints or tests; schema names and mechanisms remain subject to Phase 1 ADR/ERD review.

| Gap / implicit rule | Resolution in the plan | Verification / gate |
|---|---|---|
| Exactly-once and consistent multi-record operations | Business use cases use explicit transactions; sequence/unique constraints prevent duplicate identifiers; audit commits with the state transition. | DB integration and injected failure tests for visit, invoice, payment, stock, setup and restore. |
| History rather than destructive overwrite | Appointment status/reschedule history, finalized prescriptions, invoice price snapshots, stock movements, payment reversals, chart-per-visit and clinical addenda are retained. | Change catalog/template and prove old records/documents remain unchanged; undo/void is a new auditable record. |
| Document reprint integrity | Persisted, versioned/snapshotted document inputs and render-template version are associated with output so a reprint never relies on mutable catalog or current form state. | Finalized RX/invoice reprint after catalog/profile edits; compare persisted data and output. |
| Financial ledger reconciliation | Integer poisha, allocation rows, refund/reversal rows and defined payment/expense classification; no duplicate income aggregation. | Independent expected-value fixtures/property tests; cross-check patient balances and accounting reports. |
| Permission-safe derived data | Search result count/snippet, notification, chart, dashboard, list total, export and report are treated as data endpoints, not just UI decorations. | Direct query/IPC denial tests for every role family, including receptionist invoice capture vs analytics. |
| Duplicate patient prevention and merge | Search and duplicate warning can be offered on registration using configured name/phone/code signals; merging is not an automatic destructive action. If merge is implemented, require dedicated permission, preview, transaction, audit and referential integrity. | Duplicate registration warning; merge cancel/rollback tests. Product owner should confirm need before scope expands. |
| Initial owner account recovery | Setup creates no default password. A secure owner recovery path (prefer recovery credential generated and stored outside the normal login password) must be designed so a forgotten owner password does not strand all clinic data. | Phase 1 threat/recovery ADR and clean restart/recovery test. Must not create a hidden password reset bypass. |
| Session and local Windows user relationship | App roles are separate from Windows accounts. DPAPI key protection follows a documented OS-user/profile model; app-lock is still required even if the Windows session is unlocked. | Test app accounts under supported Windows account setup; clearly explain that app RBAC does not stop same-user local malware/admin. |
| Date, time and appointment boundary | Persist time-zone-safe instants; use Asia/Dhaka for clinic day, queue elapsed time, finance periods, appointment and audit display. Use date-only values for DOB and expiry date where appropriate. | Midnight/Dhaka-boundary, future appointment, DST/clock-change/clock-skew and restart tests. |
| Monetary display and rounding | Store integer poisha, fixed BDT, explicit rounding for percentage/line discounts/tax and localized grouping; no floating point. | Boundary/large amount/discount/allocation and PDF total tests. |
| Filesystem capacity and attachment policy | Validate file type/size and available storage; keep attachment blobs outside DB; preserve stable logical IDs, checksums, backup inclusion and clear missing/corrupt-file state. | Invalid/large/missing/corrupt file and backup/restore tests. Exact supported types/size are Phase 1/Settings policy. |
| Backups while app is shut down | “Every N days” has defined catch-up semantics; use safe per-user scheduling only if it can access key material without exposing a password. Report skipped/missed/failing jobs on next launch. | Windows scheduler and powered-off/missing-drive testing; no false schedule success. |
| Clinic closure, patient retention and data export | Ordinary uninstall preserves data. Secure export, retention, right-to-access and eventual deletion policy need documented clinic/legal review; do not erase the only backup silently. | Owner/local legal/accounting review before final release; separate permissions and audit for implemented exports. |
| Tax and invoice correction policy | Configurable tax/adjustment stays off by default; issue-time values are snapshotted. Void/credit/refund is preferred to editing a settled invoice. | Owner/accountant configures; no product claim about applicable Bangladesh tax law. |
| Staff salary/PII classification | Separate staff profile from user credential, and salary, ID/address/photo from general directory read. | Permission matrix service-level negative tests and audit of sensitive exports. |
| Printer availability/capability | Printer list, names, supported paper sizes, online state and settings come from installed Windows drivers. A paper profile can be unavailable for a particular driver and must explain this instead of silently scaling. | WebView2/Windows adapter tests plus representative physical printer. |
| Local runtime updates | WebView2/system component servicing is distinct from DCMS Pro network use; no application content/API requires the internet. Update expectations must be documented without hidden app updater. | Disconnected operation/network audit; offline installer test; version/security maintenance policy. |
| Evidence for visual claims | Headless/browser viewport tests do not establish actual Windows display scaling, native dialogs, printer output or clean installation. | Capture Windows VM screenshots, actual generated PDF and machine/driver metadata; mark unavailable evidence as blocked. |
| Commercial identity | Product creator details are supplied, but publisher/legal entity, installer signing identity, support address and trademark claims are not. | Do not fabricate; owner decides signing/publisher posture before distribution. |

## 3. Resolved conflicts and no-silent-downgrade rules

1. **“Unlimited” records vs finite device capacity:** No product row cap; does not promise physically infinite storage. State the bound and benchmark load.
2. **Offline activation vs unbreakable licensing:** A local verifier is bypassable by a determined executable analyst. Do not add a cloud check or claim DRM-grade protection.
3. **Printer flexibility vs vendor protocol:** Support installed Windows printer/driver path, not an unimplemented direct Bluetooth or proprietary interface.
4. **Financial entry vs financial confidentiality:** Grant invoice/payment entry separately from all-clinic totals/history/analytics and test all backdoors, including search and notifications.
5. **Manual merge vs PR workflow:** The agent may prepare a coherent PR from the Arena branch but cannot merge; user controls the merge and the next phase starts only after the specified Continue instruction.
6. **One final feature set vs security lifecycle:** Do not leave architecture incomplete expecting future features; retain the ability to issue a necessary security/data-loss fix. If no future security build is allowed, document owner acceptance and its residual risk.
7. **Offline use vs internet-hosted build:** Build-time package downloads may occur in GitHub Actions; the installed production app must have no mandatory runtime network dependency and must operate with network blocked.

## 4. Owner gates (not current Phase 0 blockers)

- **Before Phase 1 lock:** Confirm single-PC/no-LAN scope remains acceptable if the owner expects shared use; review the proposed SQLCipher/DPAPI backup-recovery UX; identify any must-support Windows/printer model not stated.
- **Before Phase 7/8:** Provide, if available, at least one clinic printer/driver and paper profile for real-world validation; confirm clinic-specific prescription advice, invoice footer, tax/discount and age display. These settings must not be fabricated.
- **Before Phase 13/14:** Provide or approve legal publisher/signing information and Windows signing certificate through protected CI secrets if signing is desired; perform local Bangladesh health-record/tax/privacy/retention review; provide isolated Windows and printer access if not available in CI.
- Never request passwords, certificate private keys, activation code, recovery key or other secrets in chat. Provide secret material only through an approved local/CI secret channel, or keep it out of scope.
