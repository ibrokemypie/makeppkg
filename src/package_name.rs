extern crate pkgparse_rs as pkgparse;

use std::ffi::CStr;

pub fn package_name(pkgbuild: *mut pkgparse::pkgbuild_t) -> Result<String, String> {
    let mut external_pointer = unsafe { pkgparse::pkgbuild_names(pkgbuild) };
    let mut pkgnames: Vec<&CStr> = vec![];
    if external_pointer.is_null() {
        return Err("No package name found".to_string());
    }
    loop {
        unsafe {
            let internal_pointer = *external_pointer;
            if (internal_pointer).is_null() {
                break;
            }
            pkgnames.push(&CStr::from_ptr(internal_pointer));
            external_pointer = external_pointer.add(1)
        };
    }
    return Ok(pkgnames[0].to_string_lossy().into_owned());
}
