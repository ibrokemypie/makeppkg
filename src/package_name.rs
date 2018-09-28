extern crate pkgparse_rs as pkgparse;

use c_to_r_array::c_to_r_array;

pub fn package_name(pkgbuild: *mut pkgparse::pkgbuild_t) -> Result<String, String> {
    match c_to_r_array(unsafe { pkgparse::pkgbuild_names(pkgbuild) }) {
        Ok(pkgnames) => Ok(pkgnames[0].to_owned()),
        Err(_) => Err("No package name found".to_string())
    }
}
