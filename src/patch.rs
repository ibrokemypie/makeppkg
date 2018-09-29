extern crate checksums;

use file_to_string::file_to_string;
use regex::Regex;
use std::fs::read_dir;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
struct PatchFile {
    name: String,
    path: PathBuf,
    checksum: Option<String>,
}

pub fn patch(
    location: PathBuf,
    pkgname: String,
    srcinfo: &String,
) -> Result<String, String> {
    let mut patches;
    match find_patches(location, &pkgname) {
        Ok(found_patches) => patches = found_patches,
        Err(error) => return Err(error),
    };

    patch_pkgbuild(&mut patches);

    let algorithm;
    match find_algorithm(&srcinfo) {
        Ok(found_algorithm) => algorithm = found_algorithm,
        Err(error) => return Err(error),
    };
    compute_sums(&mut patches, &algorithm);
    append_patches(&patches, &algorithm, &srcinfo);
    Ok("worked".to_string())
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

fn compute_sums(patches: &mut Vec<PatchFile>, algorithm: &checksums::Algorithm) {
    for mut patch in patches {
        patch.checksum = Some(checksums::hash_file(&patch.path, *algorithm));
        println!(
            "Patch name {}, path: {:?}, checksum: {:?}",
            &patch.name,
            &patch.path,
            &patch.checksum.to_owned().unwrap()
        );
    }
}

fn find_algorithm(srcinfo: &String) -> Result<checksums::Algorithm, String> {
    for mut line in srcinfo.lines() {
        line = line.trim();
        if line.starts_with("md5sums = ") {
            return Ok(checksums::Algorithm::MD5);
        } else if line.starts_with("sha256sums = ") {
            return Ok(checksums::Algorithm::SHA2256);
        } else if line.starts_with("sha1sums = ") {
            return Ok(checksums::Algorithm::SHA1);
        } else if line.starts_with("sha512sums = ") {
            return Ok(checksums::Algorithm::SHA2512);
        }
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

fn append_patches(
    patches: &Vec<PatchFile>,
    algorithm: &checksums::Algorithm,
    srcinfo: &String,
) {
    let mut new_sources: Vec<String> = vec![];
    {
        let re = Regex::new(r"(?mi)(?:^\s*source = )(.*)+").unwrap();
        let sources = re.captures_iter(&srcinfo);
        for source in sources {
            new_sources.push(source[1].to_string());
        }
        for patch in patches {
            new_sources.push(patch.path.to_string_lossy().to_string());
        }
    }

    let checksum;
    match algorithm {
        checksums::Algorithm::MD5 => {
            checksum = "md5sums";
        }
        checksums::Algorithm::SHA2256 => {
            checksum = "sha256sums";
        }
        checksums::Algorithm::SHA2512 => {
            checksum = "sha512sums";
        }
        checksums::Algorithm::SHA1 => {
            checksum = "sha1sums";
        }
        _ => {
            checksum = "null";
        }
    }
    let mut new_checksums: Vec<String> = vec![];
    {
        let re = Regex::new(&(r"(?mi)(?:^\s*".to_owned()+checksum+" = )(.*)+")).unwrap();
        let checksums = re.captures_iter(&srcinfo);
        for mut sum in checksums {
            new_checksums.push(sum[1].to_string());
        }
    }
    for mut patch in patches {
        new_checksums.push(patch.checksum.to_owned().unwrap());
    }

    let mut updpkgbuild = file_to_string(File::open("PKGBUILD").unwrap()).unwrap();
    updpkgbuild = replace_array_string(updpkgbuild, "source".to_string(), new_sources);
    updpkgbuild = replace_array_string(updpkgbuild, checksum.to_string(), new_checksums);
    println!("{}", updpkgbuild);
}

fn replace_array_string(text: String, name: String, new_array: Vec<String>) -> String {
    let mut start = 0;
    let mut end = 0;
    for (number, line) in text.lines().enumerate() {
        if start == 0 {
            if line.starts_with(&name) {
                start = number;
                if line.ends_with(")") {
                    end = number;
                    break;
                }
            }
        } else {
            if line.ends_with(")") {
                end = number;
                break;
            }
        }
    }
    let mut newtext: Vec<String> = text.lines().map(|s| s.to_string()).collect();
    newtext.drain(start..=end);

    let mut newstring = name + "=(";
    for item in new_array {
        newstring.push_str("\n\t'");
        newstring.push_str(&item);
        newstring.push('\'');
    }
    newstring.push_str("\n)");
    newtext.insert(start, newstring);

    newtext.join("\n")
}
