pub fn package_name(pkgbuild_contents: String) -> (Result<String, String>) {
    // Search for "pkgname=" in string
    for line in pkgbuild_contents.lines() {
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
