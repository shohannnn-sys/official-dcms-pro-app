#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(windows))]
compile_error!("This isolated proof app must be built and run on Windows.");

use std::{
    fs,
    ffi::c_void,
    path::{Path, PathBuf},
};
use tauri::{
    AppHandle, Manager,
    utils::config::WebviewUrl,
    webview::{PageLoadEvent, WebviewWindowBuilder},
};
use webview2_com::{
    Microsoft::Web::WebView2::Win32::{ICoreWebView2_16, ICoreWebView2Environment6},
    PrintToPdfStreamCompletedHandler,
};
use windows::{
    Win32::System::Com::IStream,
    core::Interface,
};

const MAX_PDF_BYTES: usize = 16 * 1024 * 1024;

fn main() {
    let Some(pdf_path) = std::env::var_os("DCMS_PROOF_PDF").map(PathBuf::from) else {
        eprintln!("DCMS_PROOF_PDF must point to a runner-temporary output file");
        std::process::exit(2);
    };
    let Some(status_path) = std::env::var_os("DCMS_PROOF_STATUS").map(PathBuf::from) else {
        eprintln!("DCMS_PROOF_STATUS must point to a runner-temporary status file");
        std::process::exit(2);
    };

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let load_app = app_handle.clone();
            let load_pdf_path = pdf_path.clone();
            let load_status_path = status_path.clone();

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into()),
            )
            .title("Phase 1 Windows Proof")
            .inner_size(640.0, 420.0)
            .on_page_load(move |webview, payload| {
                if payload.event() != PageLoadEvent::Finished {
                    return;
                }

                let callback_app = load_app.clone();
                let callback_pdf = load_pdf_path.clone();
                let callback_status = load_status_path.clone();
                let fallback_app = load_app.clone();
                let fallback_status = load_status_path.clone();
                let scheduled = webview.with_webview(move |platform_webview| {
                    if start_pdf_stream(
                        platform_webview,
                        callback_app.clone(),
                        callback_pdf,
                        callback_status.clone(),
                    )
                    .is_err()
                    {
                        write_status(&callback_status, "webview2-print-start-failed");
                        callback_app.exit(1);
                    }
                });

                if scheduled.is_err() {
                    write_status(&fallback_status, "tauri-with-webview-failed");
                    fallback_app.exit(1);
                }
            })
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|_| {
            write_status(&status_path, "tauri-runtime-failed");
            std::process::exit(1);
        });
}

fn start_pdf_stream(
    platform_webview: tauri::webview::PlatformWebview,
    app: AppHandle,
    pdf_path: PathBuf,
    status_path: PathBuf,
) -> Result<(), &'static str> {
    // Keep all raw WebView2 calls in this proof-only function. Production code would
    // isolate and review this interop in a dedicated Windows adapter.
    unsafe {
        let core_webview = platform_webview
            .controller()
            .CoreWebView2()
            .map_err(|_| "core-webview2-unavailable")?;
        let core_webview16 = core_webview
            .cast::<ICoreWebView2_16>()
            .map_err(|_| "icorewebview2-16-unavailable")?;
        let environment6 = platform_webview
            .environment()
            .cast::<ICoreWebView2Environment6>()
            .map_err(|_| "print-settings-interface-unavailable")?;
        let print_settings = environment6
            .CreatePrintSettings()
            .map_err(|_| "create-print-settings-failed")?;

        // WebView2 print dimensions are in inches. These values represent A4 exactly.
        print_settings
            .SetPageWidth(210.0 / 25.4)
            .map_err(|_| "set-a4-width-failed")?;
        print_settings
            .SetPageHeight(297.0 / 25.4)
            .map_err(|_| "set-a4-height-failed")?;
        print_settings
            .SetMarginTop(10.0 / 25.4)
            .map_err(|_| "set-a4-margin-failed")?;
        print_settings
            .SetMarginBottom(10.0 / 25.4)
            .map_err(|_| "set-a4-margin-failed")?;
        print_settings
            .SetMarginLeft(10.0 / 25.4)
            .map_err(|_| "set-a4-margin-failed")?;
        print_settings
            .SetMarginRight(10.0 / 25.4)
            .map_err(|_| "set-a4-margin-failed")?;

        let completion = PrintToPdfStreamCompletedHandler::create(Box::new(
            move |error_code, stream| -> windows::core::Result<()> {
                let outcome = if error_code.is_err() {
                    Err("webview2-pdf-stream-failed")
                } else if let Some(stream) = stream {
                    read_pdf_stream(&stream)
                } else {
                    Err("webview2-returned-empty-stream")
                };

                let exit_code = match outcome {
                    Ok(pdf_bytes) => match fs::write(&pdf_path, pdf_bytes) {
                        Ok(()) => {
                            write_status(&status_path, "success");
                            0
                        }
                        Err(_) => {
                            write_status(&status_path, "pdf-write-failed");
                            1
                        }
                    },
                    Err(status) => {
                        write_status(&status_path, status);
                        1
                    }
                };
                app.exit(exit_code);
                Ok(())
            },
        ));

        core_webview16
            .PrintToPdfStream(&print_settings, &completion)
            .map_err(|_| "start-pdf-stream-failed")
    }
}

fn read_pdf_stream(stream: &IStream) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0u8; 64 * 1024];
        let mut bytes_read = 0u32;
        // WebView2 returns an IStream rewound to the start of the completed PDF.
        let result = unsafe {
            stream.Read(
                buffer.as_mut_ptr().cast::<c_void>(),
                buffer.len() as u32,
                Some(&mut bytes_read),
            )
        };
        if result.is_err() {
            return Err("read-pdf-stream-failed");
        }
        if bytes_read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..bytes_read as usize]);
        if bytes.len() > MAX_PDF_BYTES {
            return Err("pdf-stream-exceeded-proof-limit");
        }
    }

    if bytes.len() < 5 || !bytes.starts_with(b"%PDF-") {
        return Err("pdf-stream-header-invalid");
    }
    Ok(bytes)
}

fn write_status(path: &Path, value: &str) {
    let _ = fs::write(path, format!("{value}\n"));
}
