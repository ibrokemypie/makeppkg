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
    let (options, location) = arg_parse();

    println!(
        "makeppkg directory: {}, makepkg arguments: {}",
        location.to_string_lossy(),
        options.join(" ")
    );

    // Open PKGBUILD and return an error if fails
    match File::open("PKGBUILD").map_err(|e| e.to_string()) {
        Ok(_) => {
            match cmd("makepkg", vec!["--printsrcinfo"])
                .stderr_to_stdout()
                .read()
                .map_err(|e| e.to_string())
            {
                Ok(srcinfo) => {
                    match package_name(&srcinfo) {
                        Ok(pkgname) => {
                            println!("Package name: {}", pkgname);
                            match patch(location, pkgname, &srcinfo) {
                                Ok(_) => {}
                                Err(error) => {
                                    println!("Could not run patches, continuing: {:?}", error)
                                }
                            }
                        }
                        Err(error) => {
                            println!("Could not retireve package name, continuing: {:?}", error)
                        }
                    };
                }
                Err(e) => println!("Failed run makepkg --printsrcinfo: {}", e),
            };
        }
        Err(error) => println!("Couldn't open PKGBUILD: {}", error),
    };
    // Run makepkg
    match cmd("makepkg", options)
        .stderr_to_stdout()
        .run()
        .map_err(|e| e.to_string())
    {
        Ok(_) => {}
        Err(e) => println!("Failed to run makepkg: {}", e),
    };
}
