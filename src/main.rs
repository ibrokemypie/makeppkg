extern crate libc;
extern crate duct;
extern crate pkgparse_rs as pkgparse;

mod arg_parse;
mod c_to_r_array;
mod file_to_string;
mod package_name;
mod patch;

use arg_parse::arg_parse;
use package_name::package_name;
use patch::patch;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use duct::cmd;
use std::ffi::CString;
use file_to_string::file_to_string;

fn main() {
    // Parse CLI args
    let (options, location) = arg_parse();

    print!(
        "makeppkg directory: {}, makepkg arguments: {}",
        location.to_string_lossy(),
        options.join(" ")
    );

    let pkgbuild;
    let srcinfo;
    // Open PKGBUILD and return an error if fails
    match File::open("PKGBUILD").map_err(|e| e.to_string()) {
        Ok(file) => {
            cmd("makepkg", vec!["--printsrcinfo"]).stderr_to_stdout().run();
            srcinfo = file_to_string(File::open(".SRCINFO").unwrap()).unwrap();
            pkgbuild = file_to_pkgbuild(&file);
            // Attempt to parse pkgname from pkgbuild
            // Run patch if succeed, warn on fail
            match package_name(&srcinfo) {
                Ok(pkgname) => {
                    println!(", package name: {}", pkgname);
                    match patch(location, pkgname, pkgbuild, &srcinfo) {
                        Ok(_) => {}
                        Err(error) => println!("Could not run patches, continuing: {:?}", error),
                    }
                }
                Err(error) => println!("Could not retireve package name, continuing: {:?}", error),
            };
        }
        Err(error) => println!("Couldn't open PKGBUILD: {}", error),
    };
    // Run makepkg
    cmd("makepkg", options).stderr_to_stdout().run();
}

fn file_to_pkgbuild(file: &File) -> *mut pkgparse::pkgbuild_t {
    let fd = file.as_raw_fd();
    let mode = CString::new("r").unwrap();
    let fp = unsafe { libc::fdopen(fd, mode.as_ptr()) };
    let pkgbuild = unsafe { pkgparse::pkgbuild_parse(fp) };
    return pkgbuild;
}
