mod arg_parse;
mod package_name;
mod patch;
mod run_makepkg;

use arg_parse::arg_parse;
use package_name::package_name;
use patch::patch;
use run_makepkg::run_makepkg;
use std::fs::File;
use std::io::prelude::*;


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
        Ok(file) => match pkgbuild_to_string(file) {
            Ok(pkgbuild_contents) => {
                // Attempt to parse pkgname from pkgbuild
                // Run patch if succeed, warn on fail
                match package_name(pkgbuild_contents.to_owned()) {
                    Ok(pkgname) => patch(location, pkgname, pkgbuild_contents.to_owned()),
                    Err(error) => {
                        println!("Could not retireve package name, continuing: {:?}", error)
                    }
                };
            }
            Err(error) => println!("{}", error),
        },
        Err(error) => println!("{}", error),
    };
    // Run makepkg
    run_makepkg(options);
}

fn pkgbuild_to_string(mut pkgbuild_file: File) -> (Result<String, String>) {
    let mut pkgbuild_contents = String::new();
    // Read pkgbuild to string and return error if fails
    match pkgbuild_file
        .read_to_string(&mut pkgbuild_contents)
        .map_err(|e| e.to_string())
    {
        Ok(_) => Ok(pkgbuild_contents),
        Err(error) => Err(error),
    }
}
