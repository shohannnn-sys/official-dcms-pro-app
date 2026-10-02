# Document, printing and font design

**Status:** Candidate production design aligned to ADR-0003; no product template/font, generated PDF, printer test or rendered artifact exists. A tiny WebView2 PDF adapter/static synthetic page is drafted only inside the non-shipping spike; it has not run.

## 1. Output principles

- Prescription/invoice preview, direct PDF and printer job all derive from the **same immutable, authorized persisted snapshot** and versioned local template/profile. Never print directly from mutable frontend form state as a finalized document.
- Printing and PDF export are separate explicit operations with separate permissions and completion states. No cloud converter or paid PDF/print API.
- English application chrome is default. User-entered Bengali Unicode is preserved end-to-end, including conjuncts, dependent vowels, punctuation and mixed Bengali/Latin/digits. BDT uses `৳` and two fractional digits.
- A4, A5, thermal/receipt/mini profiles and custom sizes have distinct declared geometry. A narrow document is not blindly scaled A4. A profile is offered only if the selected Windows driver supports it; otherwise tell the user and keep the record unchanged.
- “Print succeeded” is never shown solely because a job was started. Native API completion can establish submission/status; the printer/driver may still fail after accepting a job. UI wording must accurately identify the observable event.

## 2. Document model and snapshot lifecycle

### Prescription snapshot

At finalization, the backend persists:

- patient identity, patient code and age/date policy as it was selected for this issue;
- clinic name/address/phone/logo reference and configured footer/advice text;
- prescriber name plus ordered designations/qualifications/signature content as then configured;
- issue date/time in clinic zone, complaints/observations/advice, ordered medicine rows and all free text;
- paper profile, document-template version ID/hash, output-language/number format and stable render-model hash.

The snapshot is tied to the immutable prescription ID. A later dentist-profile/template/catalog change cannot rewrite a prior prescription. An authorized clinical amendment creates an explicit revision/addendum and a new document snapshot; reprint of the original remains possible subject to retention policy.

### Invoice snapshot

At issue, persist invoice number/date, clinic and patient display identity, line descriptions, quantities, unit prices, discounts, tax/adjustments, totals in integer poisha, payment status at the snapshot time if displayed, and template/paper-profile version. Later catalog/tax/profile changes do not mutate the original invoice. Paid/due values for a receipt-style reprint must be identified as issue-time snapshot or current-payment receipt; never conflate the two.

### Render versioning

A render model is a canonical typed DTO produced by Rust and stored as immutable JSON/hash with the source record. Static HTML/CSS assets are local bundle files with a code/template version. Keep the template hash/version in the record; do not execute arbitrary admin-entered JavaScript/HTML. “Deterministic” means the same persisted content/profile; WebView2 engine/runtime and system printer-driver changes can affect pixels, so exact bitwise PDF equality is not promised across runtime upgrades.

## 3. Rendering architecture

```text
finalized domain row + immutable render snapshot
   → Rust permission/policy check
   → short-lived one-use print-job lease (actor/session/document/template/profile)
   → restricted local print WebView loads bundled route and snapshot DTO
   → local CSS/font readiness checks + in-app paper preview
   → WebView2 system dialog / direct selected-printer Print / local PDF stream
   → native result mapping + safe audit + cache/job cleanup
```

- Create a dedicated restricted print WebView (or equivalent isolated print surface), not a hidden reuse of a sensitive list/profile page. Its capability includes only consuming the one-use print job and reporting safe completion; it does not receive auth/user management, arbitrary search, SQL, file access or backup commands.
- The job lease is random, short-lived, single-use and bound to the active Rust session generation, document ID/type, snapshot hash, selected template and paper profile. The backend verifies the document permission again when a job is consumed and before output. Expiry/role revocation aborts it.
- User content is escaped as text and sanitized for intended formatting; no arbitrary markup/event handler/javascript URL can execute. Logos/signatures come from validated app-managed assets. No remote URL, service worker, external script/image/font or browser cache for document payload.
- Wait for the DOM, bundled font load and image decode before enabling Print/PDF. PDF generation writes to a unique sibling `.partial`, validates completion/file size, then atomically renames; failure/cancel removes only its partial candidate.
- The printer adapter calls documented WebView2 interfaces using Tauri `with_webview` on the main thread; COM callback state is moved back to a safe application channel. No long blocking operation or DB query runs on the UI/COM thread.

## 4. Native print/PDF modes

| Mode | User experience | Observable success/failure |
|---|---|---|
| In-app preview | Paper-sized viewport with page navigation/zoom; no printer call | Preview-ready after snapshot/font/layout checks. Preview alone does not mean exported. |
| Windows system print UI | WebView2 native/browser dialog for the user to choose a printer/settings | If API only opens a dialog and provides no completion event, audit only `dialog_opened`; cancellation is no-change. Do not infer print success. |
| Direct selected-printer print | Rust enumerates Windows-installed printers and invokes WebView2 `Print` with validated selected device/settings/capability | Map `SUCCEEDED`, `PRINTER_UNAVAILABLE`, `OTHER_ERROR` and HRESULT; `E_ABORT` from an overlapping job maps to `OTHER_ERROR`. Do not invent a direct-print cancel status that WebView2 does not expose. UI says job was submitted/accepted where appropriate, not physically produced. Enforce one in-flight job per WebView. |
| Direct PDF export | User selects path through a Windows Save dialog; WebView2 `PrintToPdfStream` writes to a backend-controlled temporary file | Completion + valid non-empty output + flush/hash/atomic rename required before success. No printer driver or “Microsoft Print to PDF” is required for this path. |
| Print via Microsoft Print to PDF | Selected as one Windows-installed printer | Separate spooler path; driver/dialog may cancel/fail. Test only if driver is available; do not use as the only direct-PDF implementation. |

Printer name is the Windows system device name, not the friendly label. Query supported media settings from the selected installed driver where possible. If the driver lacks a reliable capability query, constrain the profile or defer settings to the system dialog. Never silently invent a custom form or claim all thermal hardware is supported.

## 5. Paper layout rules

- Candidate dimensions are standard A4 portrait/landscape, A5 portrait/landscape and configured narrow rolls (for example 58 mm / 80 mm widths) with driver-confirmed printable width. Store dimensions in a single internal unit (millimetres or CSS px mapping) and convert once for WebView2 settings (API dimensions are inches). Avoid inconsistent rounding between CSS `@page` and COM settings.
- Use CSS `@page` size/margins matching native print settings. Set explicit page margins, color/background policy, scale, line-height and page-break rules. Do not place critical content in unprintable edge zones.
- Prescription: clinic/dentist header, patient/date identity, complaint/exam/advice section, medicine rows, follow-up/contact, substantial signature whitespace and line; long names, qualifications and medicine instructions wrap. The signature region stays usable on A4/A5/narrow layout without pushing out required content.
- Invoice: detailed line item, quantity, rate, discount/tax, subtotal/total, paid/due and payment status; keep total rows together; no doctor signature unless explicitly configured. Reconcile every rendered amount to the integer-poisha domain totals.
- Thermal/mini: single-column, no multicolumn A4 layout, narrow-safe labels, deliberate page length/roll behavior, no hidden columns and no default scaling to a supported-but-wrong size. The selected driver may impose minimum/maximum custom dimensions; explain unsupported profiles.
- Multi-page: avoid splitting a medicine line, invoice total, patient identity header, signature line or table row where possible; define repeat headers, orphan/widow behavior, and when a page break is acceptable. Test long free-text and many rows, not only a one-line fixture.
- `prefers-reduced-motion` has no effect on static print output; print route itself has no decorative animation. Avoid reliance on hover/focus styles or screen-only controls.

## 6. Bengali font and Unicode design

- Candidate family: locally bundled **Noto Sans Bengali** for Bengali script and Noto Sans for Latin/ASCII as needed, both subject to verification from the official upstream `notofonts/noto-sans-bengali` project and exact file license. Include complete applicable SIL Open Font License 1.1 and copyright notices. Do not load from Google Fonts/CDN at runtime.
- Use local `@font-face` assets with explicit family/weight; avoid faux bold when a real weight exists. Keep a documented fallback stack for unsupported code points, but do not report success if required Bengali glyphs render as tofu or a missing-glyph box.
- Use Unicode text as stored; no lossy ASCII transliteration. Normalize identifiers only for equality/search, not clinical display text. Test combining marks, Bengali consonant conjuncts, dependent vowels, punctuation, currency, numbers, parentheses, hyphens and Latin medicine names with Bengali instructions.
- Before enabling preview/export, await `document.fonts.ready`, call `document.fonts.check` for representative Bengali and Latin samples, decode logo assets, and record the resolved profile. The preview and PDF must use identical font bytes and size weights.
- Fonts may or may not be embedded by the WebView2 PDF backend; inspect actual PDFs. If fonts are not embedded, document the target's local rendering dependency and test on a clean supported Windows machine. Do not state “embedded Bengali font” until proven.

## 7. Language, dates and money formatting

- Application UI defaults to English; user-entered Bengali remains verbatim. Do not require a full Bengali chrome translation.
- All business instants are stored UTC; document timestamps are formatted using `Asia/Dhaka` and a bundled IANA conversion library/data version. DOB and expiry remain date-only. Clinic locale setting is explicit.
- Money remains integer BDT poisha in Rust; React never performs the authoritative total. The render model supplies exact formatted display text and raw values only when a non-authoritative UI needs them. Use `৳` and two decimals consistently; no floating-point aggregate.
- Payment status on an invoice is labelled “at issue” or “current” according to its semantic source; no changing amount is implied by a historical immutable issue snapshot.

## 8. Audit and error mapping

- Log print/PDF intent, actor, source record ID/type, snapshot/template/profile hashes, selected printer/profile where safe, start/end time and observed status. No rendered text, clinical content, password, full output bytes or patient name in generic logs.
- Print/PDF permission is distinct from view permission where export risk warrants; a document preview may itself reveal protected fields and requires authorization.
- Map native error to actionable safe message: printer not installed/unavailable, unsupported size, busy operation, cancelled, disk full, inaccessible destination, WebView2 interface too old/unavailable, font readiness failure, PDF render failure. Do not show raw COM exception text to users.
- Failure or cancellation never mutates final patient, prescription, invoice or payment state. Reprinting an existing finalized snapshot is idempotent and creates an auditable new output event.

## 9. Test corpus and proof matrix (planned, not run)

Synthetic, non-identifiable fixtures only; no supplied activation code, actual patient, address, payment reference or recovery key.

| Area | Cases | Evidence |
|---|---|---|
| Typography | Bengali conjuncts/vowels/diacritics, mixed Bengali/English, Latin medicines, `৳`, long dentist qualifications/patient names | Font availability check, PDF text extraction, page-image review, WebView2/runtime/font hash. |
| Pagination | Long instructions, many medicine rows, long invoice table, multi-page totals, signature spacing, very long unbroken token | Page count, geometry, extracted text ordering, rendered images, no clipped totals or illegible body. |
| Profiles | A4/A5 both orientations where supported; 58/80 mm and configured mini size only if driver supports | Actual `@page` dimensions vs PDF metadata; printer capability result; no silent scaling. |
| Printer APIs | No printer, default printer, selected named printer, driver removed/offline/busy, unsupported media, invalid settings | Native result/status, UI safe message, audit, no domain-state change. |
| Cancellation/failure | User cancels dialog/PDF save, disk full, read-only folder, lost write permission, in-progress duplicate call, corrupt/incomplete WebView | No false success, no corrupt final PDF, safe retry, no data mutation. |
| Access control | Unauthorized role tries preview/print/PDF, stale one-use job, revoked role between preview and print | Direct Rust-service tests; no content in renderer/event/error/log. |
| Clean install | Disconnected install/runtime, first printed PDF, Bengali font presence, supported clean Windows image | Installer hash, OS/WebView2/driver versions, actual PDF + screenshot; not available in current Linux sandbox. |

## References

- [ADR-0003 WebView2 printing](adr/ADR-0003-webview2-printing.md)
- [ADR-0001 stack/runtime](adr/ADR-0001-desktop-stack.md)
- [Security/permission design](security-and-rbac.md)
- Microsoft WebView2 printing API: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/print>
- Microsoft `ICoreWebView2_16`: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/win32/icorewebview2_16>
- Noto Sans Bengali upstream: <https://github.com/notofonts/noto-sans-bengali>
- SIL Open Font License: <https://openfontlicense.org/>
- Phase 0 print/document requirements: [`../phase-0/requirements-spec.md`](../phase-0/requirements-spec.md)
