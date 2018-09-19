extern crate xdg;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    // Parse CLI args
    let (options, location) = arg_parse();

    // DEBUG
    println!(
        "Patch location: {}, makepkg arguments: {}",
        location,
        options.join(" ")
    );

    // Attempt to parse pkgname from pkgbuild
    // Run patch if succeed, warn on fail
    match package_name() {
        Ok(pkgname) => patch(pkgname),
        Err(error) => println!("Could not retireve package name, continuing: {:?}", error),
    };

    // Run makepkg
    run_makepkg(options);
}

// TODO: Patch function
fn patch(pkgname: String) {
    println!("{}", pkgname);
}

// Parses arguments from CLI
fn arg_parse() -> (Vec<String>, String) {
    // Get fallback XDG config home
    let xdg_dirs = xdg::BaseDirectories::with_prefix("makeppkg").unwrap();
    let xdghome = xdg_dirs
        .get_config_home()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut location = xdghome;

    // Store arguments
    let mut arguments = args().enumerate();
    let mut options: Vec<String> = args().collect();

    // Find -f
    if arguments.find(|(_, x)| x == &"-f".to_string()) != None {
        let string = arguments.next();
        if string.is_some() {
            // Remove -f and following value from arguments that are passed to makepkg
            let unwrapped = string.unwrap();
            let index = unwrapped.0;
            options.remove(index);
            options.remove(index - 1);
            // Store value after -f as new location
            location = unwrapped.1;
        } else {
            // Fail if -f provided with no location
            println!("Provide a location when using the -l option");
            process::exit(1);
        }
    }
    // Remove first argument (executable path)
    options.remove(0);

    return (options, location);
}

fn package_name() -> (Result<String, String>) {
    let mut contents = String::new();

    // Open PKGBUILD and return an error if fails
    let mut pkgbuild = File::open("PKGBUILD").map_err(|e| e.to_string())?;
    // Read pkgbuild to string and return error if fails
    pkgbuild
        .read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    // Search for "pkgname=" in string
    for line in contents.lines() {
        if line.starts_with("pkgname=") {
            // Remove "pkgname=" from resultant string
            let (_, mut value) = line.split_at(8);
            // Logic for multiple names
            if value.starts_with("(") {
                // Remove "(", ")" and "'" fron string
                let matches = ['(', ')', '\''];
                let names = value
                    .chars()
                    .filter(|m| !matches.contains(m))
                    .collect::<String>();
                // Split into array of names
                let pkgname = names.split(", ").next().unwrap();
                // Return first name
                return Ok(pkgname.to_string());
            }
            // Return name
            let pkgname = value;
            return Ok(pkgname.to_string());
        }
    }
    // Return error if no names found
    return Err("No package name found".to_string());
}

// TODO: run makepkg instead of echo
fn run_makepkg(options: Vec<String>) {
    let output = process::Command::new("echo")
        .args(options)
        .output()
        .expect("Failed");
    // TODO: connect stdout and stderr streams
    if output.status.success() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
