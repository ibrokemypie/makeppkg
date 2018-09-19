mod arg_parse;
mod package_name;
mod run_makepkg;
mod patch;

use arg_parse::arg_parse;
use package_name::package_name;
use run_makepkg::run_makepkg;
use patch::patch;

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
