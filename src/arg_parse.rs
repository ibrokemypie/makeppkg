extern crate xdg;

use std::env::args;
use std::path::PathBuf;
use std::process;

// Parses arguments from CLI
pub fn arg_parse() -> (Vec<String>, PathBuf, String) {
    // Get fallback XDG config home
    let xdg_dirs = xdg::BaseDirectories::with_prefix("makeppkg").unwrap();
    let xdghome = xdg_dirs
        .get_config_home()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut location = PathBuf::from(xdghome);
    let mut pkgbuild_path = "PKGBUILD".to_string();

    let mut options: Vec<String> = args().collect();

    // Find -f
    let mut arguments = args().enumerate();
    if arguments.find(|(_, x)| x == &"-l".to_string()) != None {
        let string = arguments.next();
        if string.is_some() {
            // Remove -l and following value from arguments that are passed to makepkg
            let unwrapped = string.unwrap();
            let index = unwrapped.0;
            options.remove(index);
            options.remove(index - 1);
            // Store value after -l as new location
            location = PathBuf::from(unwrapped.1);
        } else {
            // Fail if -l provided with no location
            eprintln!("Provide a location when using the -l option");
            process::exit(1);
        }
    }

    let mut arguments = args().enumerate();
    if arguments.find(|(_, x)| x == &"-p".to_string()) != None {
        println!("found -p");
        let string = arguments.next();
        if string.is_some() {
            // Remove -l and following value from arguments that are passed to makepkg
            let unwrapped = string.unwrap();
            // Store value after -l as new location
            pkgbuild_path = unwrapped.1;
        }
    }
    // Remove first argument (executable path)
    options.remove(0);

    return (options, location, pkgbuild_path);
}
