extern crate libc;
extern crate pkgparse_rs as pkgparse;

mod arg_parse;
mod file_to_string;
mod package_name;
mod patch;
mod run_makepkg;

use arg_parse::arg_parse;
use package_name::package_name;
use patch::patch;
use run_makepkg::run_makepkg;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use std::ffi::CString;

fn main() {
    // Parse CLI args
    let (options, location) = arg_parse();

    // DEBUG
    println!(
        "makeppkg directory: {}, makepkg arguments: {}",
        location.to_string_lossy(),
        options.join(" ")
    );

    // Open PKGBUILD and return an error if fails
    match File::open("PKGBUILD").map_err(|e| e.to_string()) {
        Ok(file) => {
            let pkgbuild = file_to_pkgbuild(&file);
            // Attempt to parse pkgname from pkgbuild
            // Run patch if succeed, warn on fail
            match package_name(pkgbuild) {
                Ok(pkgname) => patch(location, pkgname, pkgbuild),
                Err(error) => println!("Could not retireve package name, continuing: {:?}", error),
            };
        }
        Err(error) => println!("{}", error),
    };
    // Run makepkg
    run_makepkg(options);
}

fn file_to_pkgbuild(file: &File) -> *mut pkgparse::pkgbuild_t {
    let fd = file.as_raw_fd();
    let mode = CString::new("r").unwrap();
    let fp = unsafe { libc::fdopen(fd, mode.as_ptr()) };
    let pkgbuild = unsafe { pkgparse::pkgbuild_parse(fp) };
    return pkgbuild;
}
