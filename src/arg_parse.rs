extern crate xdg;

use std::env::args;
use std::path::PathBuf;
use std::process::exit;

// Parses arguments from CLI
pub fn arg_parse() -> (Vec<String>, Option<(PathBuf, PathBuf)>) {
    // Get fallback XDG config home
    let xdg_dirs = xdg::BaseDirectories::with_prefix("makeppkg").unwrap();
    let xdghome = xdg_dirs
        .get_config_home()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut location = PathBuf::from(xdghome);
    let mut pkgbuild_path = PathBuf::from("PKGBUILD");

    let blacklist = vec![
        "noextract",
        "verifysource",
        "geninteg",
        "repackage",
        "source",
        "version",
        "allsource",
        "packagelist",
        "printsrcinfo",
    ];

    let mut options: Vec<String> = args().collect();
    options.remove(0);

    let iter = options.to_owned();
    let mut iter = iter.iter().enumerate().peekable();
    while let Some((i, value)) = iter.next() {
        if blacklist.contains(&value.trim_left_matches("-")) {
            return (options, None);
        }
        if &value == &"-l" {
            if iter.peek().is_some() {
                location = PathBuf::from(iter.peek().unwrap().1);
                options.remove(i + 1);
                options.remove(i);
            } else {
                eprintln!("Specify makeppkg location when using -l flag.");
                exit(1);
            }
        }
        if &value == &"-p" {
            if iter.peek().is_some() {
                pkgbuild_path = PathBuf::from(iter.peek().unwrap().1);
            }
        }
    }

    return (options, Some((location, pkgbuild_path)));
}
