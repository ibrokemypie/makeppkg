extern crate digest;
extern crate duct;
extern crate md5;
extern crate regex;
extern crate sha1;
extern crate sha2;

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
use std::fs::OpenOptions;
use std::io::Write;

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
        match OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(&pkgbuild_path)
            .map_err(|e| e.to_string())
        {
            Ok(mut pkgbuild_file) => {
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
                                let mut pkgbuild_contents = vec![];

                                match patch(
                                    location,
                                    pkgname,
                                    &srcinfo,
                                    &pkgbuild_path,
                                    &mut pkgbuild_file,
                                ) {
                                    Ok(new_pkgbuild_contents) => {
                                        pkgbuild_contents = new_pkgbuild_contents;
                                    }
                                    Err(error) => {
                                        eprintln!("Could not run patches, continuing: {:?}", error)
                                    }
                                }
                                // Run makepkg
                                match cmd("makepkg", options).stderr_to_stdout().run() {
                                    Ok(_) => {}
                                    Err(error) => eprintln!("Failed to run makepkg: {}", error),
                                };
                                if pkgbuild_contents.len() != 0 {
                                    pkgbuild_file.write_all(&pkgbuild_contents).unwrap();
                                    eprintln!("PKGBUILD restored.")
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
    } else {
        // Run makepkg
        match cmd("makepkg", options).stderr_to_stdout().run() {
            Ok(_) => {}
            Err(error) => eprintln!("Failed to run makepkg: {}", error),
        };
    }
}
