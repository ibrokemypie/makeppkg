extern crate xdg;

use std::env::args;
use std::process;
use std::path::PathBuf;

// Parses arguments from CLI
pub fn arg_parse() -> (Vec<String>, PathBuf) {
    // Get fallback XDG config home
    let xdg_dirs = xdg::BaseDirectories::with_prefix("makeppkg").unwrap();
    let xdghome = xdg_dirs
        .get_config_home()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut location = PathBuf::from(xdghome);

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
            location = PathBuf::from(unwrapped.1);
        } else {
            // Fail if -f provided with no location
            println!("Provide a location when using the -f option");
            process::exit(1);
        }
    }
    // Remove first argument (executable path)
    options.remove(0);

    return (options, location);
}
