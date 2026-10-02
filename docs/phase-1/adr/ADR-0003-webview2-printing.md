# ADR-0003 — WebView2 document rendering and Windows printing

- **Status:** Provisional; Microsoft API availability is documented, but no Tauri/Rust Windows runtime, PDF, driver or Bengali-output proof has run.
- **Date:** 2026-10-02 (UTC)
- **Decision owner:** Product owner
- **Related decisions:** ADR-0001 (stack), ADR-0004 (backup)

## Context

Prescriptions and invoices require a real preview, A4/A5 and supported narrow/thermal layouts, printer selection, direct PDF export, Bengali Unicode shaping and predictable pagination. The renderer must use persisted, authorized document data. Only printers installed in Windows through supported drivers are in baseline scope; direct Bluetooth/USB protocols are not.

Tauri's high-level `WebviewWindow::print()` currently documents that its dialog path is supported on macOS/Wry, while `window.print()` is available across platforms. That is insufficient evidence for selected-printer settings, custom sizes and reliable PDF completion. Microsoft's WebView2 API is more complete, but Tauri does not ship a first-party end-to-end printer-management/document-publishing subsystem.

## Proposed decision

Use **local HTML/CSS document templates rendered by the same pinned WebView2 runtime**, with a small Windows-only Rust adapter through Tauri's `with_webview` API.

Candidate WebView2 interfaces:

- `ICoreWebView2_16::ShowPrintUI` to offer the WebView2 browser or native Windows print dialog.
- `ICoreWebView2_16::Print` plus `ICoreWebView2Environment6::CreatePrintSettings` for direct printing to a selected installed printer with completion status and explicit page settings.
- `ICoreWebView2_16::PrintToPdfStream` (or an equivalent pinned, tested stream API) for local PDF generation into a controlled save flow; PDF output is a separate operation from sending a job to a printer.
- Windows printer enumeration/capability APIs to populate printer names and supported paper options. A UI profile may not claim a custom size that the selected driver does not support.

The specific interfaces and minimum WebView2 SDK/runtime version must be pinned only after the Tauri Windows spike compiles and runs. `with_webview` runs its closure on the main thread; asynchronous COM completion must not block that thread. All `unsafe` interop is confined to one reviewed module and translated into typed internal status values.

### Document flow

1. Authorized application service selects a **persisted, finalized** prescription/invoice/document snapshot. The renderer cannot submit arbitrary patient, money or clinical data as the source of a finalized document.
2. Rust checks the actor's `documents.preview`, `documents.print` or `documents.pdf.export` permission and creates a short-lived, single-use `print_job` bound to the active session generation, document ID/type, immutable snapshot hash, selected template version and paper profile.
3. A separate, restricted local print WebView loads only a bundled print route. It consumes the job once; no remote navigation/assets/JS are allowed. The route receives a minimal authorized render DTO, not SQL results, local file paths, secrets or general commands.
4. Wait for document readiness and local fonts (`document.fonts.ready` plus explicit font checks), then show an in-app preview at the selected paper geometry. If required glyph coverage or asset readiness fails, abort with an actionable error rather than silently rendering a fallback.
5. User may print via the Windows system dialog or direct selected-printer settings, or export PDF via a native save dialog and WebView2 PDF stream. Write PDF output to a sibling `.partial`, validate completion/size and rename atomically only after a successful render. A printer API success means the job was submitted/accepted by Windows, not that paper physically emerged; UI wording must say “sent to printer/spooler,” not guarantee physical output.
6. Audit the requested action and result/status without logging document text. A cancelled dialog/export or printer failure does not change the finalized record and never reports a false success. Whether the system-dialog route reports user cancellation is a spike question; when the API provides no completion callback, log only that the dialog was opened, not a completed print.

## Paper and font rules

- A4/A5 use explicit page-box dimensions and margins; thermal/receipt/mini templates are separately laid out and may be offered only when the selected driver supports the width/height. Never silently scale A4 into receipt width.
- `@page` rules and `CoreWebView2PrintSettings` must use the same normalized profile. `break-inside: avoid`, explicit page breaks, line-height, table headers/repeats, long-string wrapping and signature reserves are tested per document type.
- Local `Noto Sans`/`Noto Sans Bengali` files are candidates under SIL Open Font License 1.1. Exact upstream files, coverage, copyright, `OFL.txt`, embedding/subsetting permissions and notices must be verified before inclusion. Do not download fonts at runtime.
- Preserve Bengali grapheme clusters/conjuncts and mixed Bengali/Latin/digit punctuation. Test `৳` and two-decimal BDT amounts, long names/qualifications/medicine instructions, and PDF text extraction plus rasterized visual review. Do not claim a font/render pass from an ASCII-only test.

## Alternatives

| Alternative | Decision |
|---|---|
| `window.print()` only | Fallback for a basic user-triggered dialog if the native adapter fails. It does not establish selected-printer enumeration, custom settings, direct PDF completion or reliable cancellation state. It cannot be the sole acceptance path. |
| Tauri high-level `WebviewWindow::print()` | Not relied upon for Windows; its current docs say this dialog method is supported only on macOS in Wry. |
| Electron `webContents.print/getPrintersAsync/printToPDF` | Strong fallback; use if a measured Tauri/WebView2 integration defect cannot be repaired safely. Re-evaluate renderer/runtime security and packaged size. |
| WPF native `PrintDialog`/XPS plus WebView2 | Strong Windows-native alternative; still requires a proven HTML/Bengali/PDF path and compatible no-paid SQLCipher stack. |
| Paid cloud PDF/print service | Rejected: recurring/network dependency and clinical-data exposure violate constraints. |

## Consequences and risks

- Rust COM bindings are version-coupled and may be unsafe; pin Tauri minor and WebView2 COM dependency, isolate unsafe code, and run Windows integration tests after every update.
- WebView2 `Print` reports `SUCCEEDED`, `PRINTER_UNAVAILABLE` or `OTHER_ERROR`; an overlapping operation is `E_ABORT`/`OTHER_ERROR`. The documented API has no distinct user-cancel status for direct printing. Driver success is not physical-output proof. `ShowPrintUI` may not give the application a completion/cancel event; no audit record may assert more than was observed.
- Thermal roll sizes, user-installed font support and printer-specific margins vary by driver. Unsupported profiles must be disabled or clearly refused, never silently substituted.
- PDF content and direct printer content must originate from the same immutable snapshot and CSS profile. A PDF generated successfully does not prove a physical printer produces the same artifact.

## Required Windows proof before acceptance

- Compile against the candidate Tauri and current `webview2-com`/`windows` bindings on x64 MSVC.
- Run a small actual Tauri app with a local HTML test document through the system dialog, direct selected-printer API and PDF stream. Exercise Microsoft Print to PDF if available and one representative physical clinic printer if hardware can be provided.
- Verify A4 and A5 dimensions, supported 58/80 mm driver profiles, copies/duplex/margins as exposed, cancellation/unavailable/busy/disk-full/write failure, output metadata and text extraction.
- Test Bengali script with local licensed fonts, long mixed-language prescription/invoice text, multipage output, page breaks, totals and signature spacing. Record OS build, WebView2 runtime version, API crate versions, printer/driver, PDF hashes and actual outcomes.
- If `ShowPrintUI` cannot signal cancellation, document the honest user-visible semantics and offer a no-change cancel path before starting the job.

## References

- Microsoft WebView2 print, print-to-PDF and print settings: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/print>
- Win32 `ICoreWebView2_16`: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/win32/icorewebview2_16>
- `ICoreWebView2PrintSettings` and custom page dimensions: <https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/win32/icorewebview2printsettings>
- Tauri `with_webview` threading/version note: <https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html>
- Tauri high-level print API limitations: <https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html#method.print>
- SQLCipher/Tauri comparison and Phase 0 print gates: [`../../phase-0/architecture-candidates.md`](../../phase-0/architecture-candidates.md)
