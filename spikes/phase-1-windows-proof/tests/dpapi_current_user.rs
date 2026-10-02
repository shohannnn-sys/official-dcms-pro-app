use std::{ffi::c_void, ptr, slice};

#[repr(C)]
struct DataBlob {
    cb_data: u32,
    pb_data: *mut u8,
}

const CRYPTPROTECT_UI_FORBIDDEN: u32 = 0x1;
const PROOF_BYTES: &[u8] = b"phase1-dpapi-proof-only-key-material";

#[link(name = "Crypt32")]
unsafe extern "system" {
    fn CryptProtectData(
        data_in: *const DataBlob,
        description: *const u16,
        optional_entropy: *const DataBlob,
        reserved: *const c_void,
        prompt: *const c_void,
        flags: u32,
        data_out: *mut DataBlob,
    ) -> i32;

    fn CryptUnprotectData(
        data_in: *const DataBlob,
        description_out: *mut *mut u16,
        optional_entropy: *const DataBlob,
        reserved: *const c_void,
        prompt: *const c_void,
        flags: u32,
        data_out: *mut DataBlob,
    ) -> i32;
}

#[link(name = "Kernel32")]
unsafe extern "system" {
    fn LocalFree(memory: *mut c_void) -> *mut c_void;
}

unsafe fn free_blob(blob: &mut DataBlob) {
    if !blob.pb_data.is_null() {
        // DPAPI documents LocalFree as the required deallocator for returned DATA_BLOB bytes.
        unsafe { LocalFree(blob.pb_data.cast::<c_void>()) };
        blob.pb_data = ptr::null_mut();
        blob.cb_data = 0;
    }
}

#[test]
fn dpapi_current_user_round_trip_and_tamper_rejection() {
    let input = DataBlob {
        cb_data: PROOF_BYTES.len() as u32,
        pb_data: PROOF_BYTES.as_ptr().cast_mut(),
    };
    let mut protected = DataBlob {
        cb_data: 0,
        pb_data: ptr::null_mut(),
    };

    let protected_ok = unsafe {
        CryptProtectData(
            &input,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut protected,
        )
    };
    assert_ne!(protected_ok, 0, "CryptProtectData failed for the current user");
    assert!(!protected.pb_data.is_null());

    let protected_bytes = unsafe {
        slice::from_raw_parts(protected.pb_data, protected.cb_data as usize).to_vec()
    };
    assert_ne!(
        protected_bytes.as_slice(),
        PROOF_BYTES,
        "DPAPI output contains the plaintext proof value"
    );
    unsafe { free_blob(&mut protected) };

    let mut tampered = protected_bytes.clone();
    tampered[0] ^= 0x80;
    let tampered_blob = DataBlob {
        cb_data: tampered.len() as u32,
        pb_data: tampered.as_mut_ptr(),
    };
    let mut rejected_output = DataBlob {
        cb_data: 0,
        pb_data: ptr::null_mut(),
    };
    let tampered_ok = unsafe {
        CryptUnprotectData(
            &tampered_blob,
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut rejected_output,
        )
    };
    if tampered_ok != 0 {
        unsafe { free_blob(&mut rejected_output) };
    }
    assert_eq!(tampered_ok, 0, "tampered DPAPI data was accepted");

    let protected_blob = DataBlob {
        cb_data: protected_bytes.len() as u32,
        pb_data: protected_bytes.as_ptr().cast_mut(),
    };
    let mut clear = DataBlob {
        cb_data: 0,
        pb_data: ptr::null_mut(),
    };
    let clear_ok = unsafe {
        CryptUnprotectData(
            &protected_blob,
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut clear,
        )
    };
    assert_ne!(clear_ok, 0, "CryptUnprotectData failed for the same user profile");
    let clear_bytes = unsafe { slice::from_raw_parts(clear.pb_data, clear.cb_data as usize) };
    assert_eq!(clear_bytes, PROOF_BYTES);
    unsafe { free_blob(&mut clear) };
}
