extern crate libc;
extern crate duct;
extern crate regex;

mod arg_parse;
mod file_to_string;
mod package_name;
mod patch;

use arg_parse::arg_parse;
use package_name::package_name;
use patch::patch;
use std::fs::File;
use duct::cmd;
use file_to_string::file_to_string;

fn main() {
    // Parse CLI args
    let (options, location) = arg_parse();

    print!(
        "makeppkg directory: {}, makepkg arguments: {}",
        location.to_string_lossy(),
        options.join(" ")
    );

    let srcinfo;
    // Open PKGBUILD and return an error if fails
    match File::open("PKGBUILD").map_err(|e| e.to_string()) {
        Ok(_) => {
            cmd("makepkg", vec!["--printsrcinfo"]).stderr_to_stdout().run();
            srcinfo = file_to_string(File::open(".SRCINFO").unwrap()).unwrap();
            // Attempt to parse pkgname from pkgbuild
            // Run patch if succeed, warn on fail
            match package_name(&srcinfo) {
                Ok(pkgname) => {
                    println!(", package name: {}", pkgname);
                    match patch(location, pkgname, &srcinfo) {
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
