pub fn package_name(srcinfo: &String) -> Result<String, String> {
    for mut line in srcinfo.lines() {
        if line.starts_with("pkgbase = ") {
            let (_, pkgname) = line.split_at(10);
            return Ok(pkgname.to_owned());
        }
    }
    Err("No package name found".to_string())
}
