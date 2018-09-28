extern crate checksums;
extern crate pkgparse_rs as pkgparse;

use file_to_string::file_to_string;
use std::fs::read_dir;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use std::ffi::CStr;

#[derive(Debug)]
struct PatchFile {
    name: String,
    path: PathBuf,
    checksum: Option<String>,
}

pub fn patch(location: PathBuf, pkgname: String, pkgbuild: *mut pkgparse::pkgbuild_t) {
    match find_patches(location, &pkgname) {
        Ok(patches) => compute_sums(patches, pkgbuild),
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
        let patch_name = patch_name(patch_path.to_owned());
        if patch_path.to_string_lossy().ends_with(".patch") {
            let mut patch = PatchFile {
                name: patch_name,
                path: patch_path,
                checksum: None,
            };
            patches.push(patch);
        }
    }
    Ok(patches)
}

fn patch_name(patch_path: PathBuf) -> String {
    match File::open(patch_path.to_owned()).map_err(|e| e.to_string()) {
        Ok(patchfile) => match file_to_string(patchfile) {
            Ok(patch) => {
                let line = patch.lines().next();
                if line.is_some() {
                    let (_, split) = line.unwrap().split_at(3);
                    let name = split.split_whitespace().next().unwrap();
                    return name.to_string();
                }
            }
            Err(error) => println!("{}", error),
        },
        Err(error) => println!("{}", error),
    };
    return patch_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
}

fn compute_sums(mut patches: Vec<PatchFile>, pkgbuild: *mut pkgparse::pkgbuild_t) {
    match find_algorithm(pkgbuild) {
        Ok(algorithm) => {
            for patch in &mut patches {
                patch.checksum = Some(checksums::hash_file(&patch.path, algorithm));
                println!(
                    "Patch name {}, path: {:?}, checksum: {:?}",
                    &patch.name,
                    &patch.path,
                    &patch.checksum.to_owned().unwrap()
                );
            }
            patch_pkgbuild(&mut patches);
            append_patches(pkgbuild, &patches);
        }
        Err(error) => println!("{}", error),
    };
}

fn find_algorithm(pkgbuild: *mut pkgparse::pkgbuild_t) -> Result<checksums::Algorithm, String> {
    if unsafe { !pkgparse::pkgbuild_md5sums(pkgbuild).is_null() } {
        return Ok(checksums::Algorithm::MD5);
    } else if unsafe { !pkgparse::pkgbuild_sha256sums(pkgbuild).is_null() } {
        return Ok(checksums::Algorithm::SHA2256);
    } else if unsafe { !pkgparse::pkgbuild_sha1sums(pkgbuild).is_null() } {
        return Ok(checksums::Algorithm::SHA1);
    } else if unsafe { !pkgparse::pkgbuild_sha512sums(pkgbuild).is_null() } {
        return Ok(checksums::Algorithm::SHA2512);
    }
    Err("No algorithm found".to_string())
}

fn patch_pkgbuild(patches: &mut Vec<PatchFile>) {
    match patches
        .iter()
        .position(|item| item.name.eq_ignore_ascii_case("PKGBUILD"))
    {
        Some(i) => match Command::new("patch")
            .arg("-i")
            .arg(&patches[i].path)
            .output()
            .map_err(|e| e.to_string())
        {
            Ok(output) => {
                if output.status.success() {
                    println!("PKGBUILD patched successfully.");
                    patches.remove(i);
                } else {
                    println!("Failed to patch PKGBUILD:");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(error) => println!("{}", error),
        },
        None => {
            println!("No patches for PKGBUILD found, continuing.");
        }
    }
}

fn append_patches(pkgbuild: *mut pkgparse::pkgbuild_t, patches: &Vec<PatchFile>) {
    let mut external_pointer = unsafe { pkgparse::pkgbuild_sources(pkgbuild) };
    let mut sources: Vec<PathBuf> = vec![];
    if !external_pointer.is_null() {
        loop {
            unsafe {
                let internal_pointer = *external_pointer;
                if (internal_pointer).is_null() {
                    break;
                }
                sources.push(PathBuf::from(
                    CStr::from_ptr(internal_pointer)
                        .to_string_lossy()
                        .into_owned(),
                ));
                external_pointer = external_pointer.add(1)
            };
        }
        println!("Sources: {:?}", sources);
    } else {
        println!("No sources found.");
    }
    for patch in patches {
        sources.push(patch.path.to_owned())
    }
    println!("Sources: {:?}", sources);
}
