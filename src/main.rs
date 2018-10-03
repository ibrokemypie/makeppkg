extern crate duct;
extern crate libc;
extern crate regex;

#[macro_use]
mod macros;
mod arg_parse;
mod file_to_string;
mod package_name;
mod patch;

use arg_parse::arg_parse;
use duct::cmd;
use package_name::package_name;
use patch::patch;
use std::fs::File;

fn main() {
    // Parse CLI args
    let (options, other) = arg_parse();

    if other.is_some() {
        let (location, pkgbuild_path) = other.unwrap();
        eprintln!(
            "makeppkg directory: {}, makepkg arguments: {}, pkgbuild path: {}",
            location.to_string_lossy(),
            options.join(" "),
            pkgbuild_path.to_string_lossy()
        );

        // Open PKGBUILD and return an error if fails
        match File::open(&pkgbuild_path).map_err(|e| e.to_string()) {
            Ok(_) => {
                match cmd(
                    "makepkg",
                    vec![
                        "--printsrcinfo",
                        "-p",
                        format!("{}", &pkgbuild_path.to_string_lossy()).as_str(),
                    ],
                ).stderr_to_stdout()
                .read()
                {
                    Ok(srcinfo) => {
                        match package_name(&srcinfo) {
                            Ok(pkgname) => {
                                eprintln!("Package name: {}", pkgname);
                                match patch(location, pkgname, &srcinfo, &pkgbuild_path) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        eprintln!("Could not run patches, continuing: {:?}", error)
                                    }
                                }
                            }
                            Err(error) => eprintln!(
                                "Could not retireve package name, continuing: {:?}",
                                error
                            ),
                        };
                    }
                    Err(error) => eprintln!("Failed run makepkg --printsrcinfo: {:?}", error),
                };
            }
            Err(error) => eprintln!("Couldn't open PKGBUILD: {}", error),
        };
    }

    // Run makepkg
    match cmd("makepkg", options).stderr_to_stdout().run() {
        Ok(_) => {}
        Err(error) => eprintln!("Failed to run makepkg: {}", error),
    };
}
