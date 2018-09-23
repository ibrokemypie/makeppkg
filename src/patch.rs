extern crate checksums;

use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Debug)]
struct PatchFile {
    path: PathBuf,
    checksum: Option<String>,
}

pub fn patch(location: PathBuf, pkgname: String, pkgbuild_contents: String) {
    match find_patches(location, &pkgname) {
        Ok(patches) => compute_sums(patches, pkgbuild_contents),
        Err(error) => println!("No patches were able to be found: {}", error),
    }
}

fn find_patches(location: PathBuf, pkgname: &String) -> (Result<Vec<PatchFile>, String>) {
    let patch_path = location.join(pkgname);
    let patch_path_string = patch_path.to_string_lossy().into_owned();
    println!("Searching for patches in {}", patch_path_string);

    let mut patches: Vec<PatchFile> = Vec::new();
    for entry in
        read_dir(patch_path).map_err(|e| format!("{}: {}", e.to_string(), patch_path_string))?
    {
        let patch_path = entry.map_err(|e| e.to_string())?.path();
        if patch_path.to_string_lossy().ends_with(".patch") {
            let mut patch = PatchFile {
                path: patch_path,
                checksum: None,
            };
            patches.push(patch);
        }
    }
    Ok(patches)
}

fn compute_sums(patches: Vec<PatchFile>, pkgbuild_contents: String) {
    match find_algorithm(pkgbuild_contents) {
        Ok(algorithm) => for mut patch in patches {
            patch.checksum = Some(checksums::hash_file(&patch.path, algorithm));
            println!(
                "Patch path: {:?}, checksum: {:?}",
                patch.path,
                patch.checksum.unwrap()
            );
        },
        Err(error) => println!("{}", error),
    };
}

fn find_algorithm(pkgbuild_contents: String) -> Result<checksums::Algorithm, String> {
    for line in pkgbuild_contents.lines() {
        if line.starts_with("md5sums=") {
            return Ok(checksums::Algorithm::MD5);
        } else if line.starts_with("sha256sums=") {
            return Ok(checksums::Algorithm::SHA2256);
        } else if line.starts_with("sha1sums=") {
            return Ok(checksums::Algorithm::SHA1);
        } else if line.starts_with("sha512sums=") {
            return Ok(checksums::Algorithm::SHA2512);
        }
    }
    Err("No algorithm found".to_string())
}
