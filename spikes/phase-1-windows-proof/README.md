# Phase 1 Windows architecture proof (non-shipping)

This isolated harness is a **technical spike only**. It is not product code, an installer, seed data, a test account, or a supported application. Nothing here is part of a release bundle.

## What the Windows workflow exercises

- Builds the exact locked proof crate on `windows-2022` using the MSVC target and runs Rust tests for bundled SQLCipher, keyed wrong-key rejection, database/WAL ciphertext marker checks, same-key encrypted online backup, and Windows DPAPI CurrentUser round-trip/tamper rejection.
- Starts a minimal real Tauri/Wry window with a local static HTML page; casts the active WebView2 object to `ICoreWebView2_16`; creates A4 print settings; calls `PrintToPdfStream`; writes the returned `IStream` to a temporary PDF; and checks the PDF header, one-page A4 geometry, and extracted English/Bengali/BDT text with pinned test-only `pypdf`.
- Keeps the synthetic key and document text fixed to test-only constants. They are not production credentials or clinical data.

## What it does not prove

- A hosted Windows Server 2022 job is not a clean Windows 11 customer install. It does not validate offline Evergreen runtime provisioning, NSIS install/uninstall/upgrade, disconnected provisioning, Windows 11 high-DPI acceptance, or per-user install policy.
- It does not prove SQLCipher/DPAPI behavior across different Windows user profiles, `age` backup recovery, full staged restore, key rotation, or scheduled-task behavior.
- The PDF path proves a local WebView2 PDF stream and text extraction, not visible glyph quality. No PDF raster comparison, licensed bundled Bengali font, selected-printer enumeration, native print dialog, physical printer, 58/80 mm driver profile, spool completion, or paper quality is proven by this harness.
- No result is recorded as passed until the workflow has actually run and its logs/artifacts are reviewed.

## Workflow outputs

The pinned GitHub Actions workflow is `.github/workflows/phase-1-windows-proof.yml`. It retains a short-lived test artifact containing the synthetic PDF, environment versions and status. It never receives signing, activation or recovery secrets.
