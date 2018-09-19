use std::fs::File;
use std::io::prelude::*;

pub fn package_name() -> (Result<String, String>) {
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
