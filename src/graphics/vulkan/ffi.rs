use std::ffi::{c_char, CString};

/// Build a vector of CStrings and a matching vector of pointers to those
/// strings.
///
/// # Safety
///
/// Unsafe because:
///   - The pointers are only valid so long as the returned strings are not
///     dropped or modified.
pub unsafe fn to_os_ptrs(
    strings: &[String],
) -> (Vec<CString>, Vec<*const c_char>) {
    let cstrings = strings
        .iter()
        .cloned()
        .map(|str| CString::new(str).unwrap())
        .collect::<Vec<CString>>();
    let ptrs = cstrings
        .iter()
        .map(|cstr| cstr.as_ptr())
        .collect::<Vec<*const c_char>>();
    (cstrings, ptrs)
}
