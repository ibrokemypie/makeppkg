use duct::cmd;
use file_to_string::file_to_string;
use md5::Md5;
use digest::Digest;
use regex::Regex;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fs::read;
use std::fs::{copy, read_dir, read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::{PathBuf,Path};

#[derive(Debug)]
struct PatchFile {
    name: String,
    path: PathBuf,
    checksum: Option<String>,
}

struct Algorithm {
    name: String,
    function: fn(&Path) -> String,
}

pub fn patch(
    location: PathBuf,
    pkgname: String,
    srcinfo: &String,
    pkgbuild_path: &PathBuf,
) -> Result<String, String> {
    let mut patches;
    match find_patches(location, &pkgname) {
        Ok(found_patches) => patches = found_patches,
        Err(error) => return Err(error),
    };

    if patches.len() != 0 {
        patch_pkgbuild(&mut patches);

        if patches.len() != 0 {
            let algorithm;
            match find_algorithm(&srcinfo) {
                Ok(found_algorithm) => algorithm = found_algorithm,
                Err(error) => return Err(error),
            };
            compute_sums(&mut patches, &algorithm);
            append_patches(&patches, &algorithm, &srcinfo, &pkgbuild_path);
        }
    }
    Ok("worked".to_string())
}

fn find_patches(location: PathBuf, pkgname: &String) -> (Result<Vec<PatchFile>, String>) {
    let patch_path = location.join(pkgname);
    let patch_path_string = patch_path.to_string_lossy().into_owned();
    eprintln!("Searching for patches in {}", patch_path_string);

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
            Err(error) => eprintln!("{}", error),
        },
        Err(error) => eprintln!("{}", error),
    };
    return patch_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
}

fn compute_sums(patches: &mut Vec<PatchFile>, algorithm: &Algorithm) {
    for mut patch in patches {
        patch.checksum = Some((algorithm.function)(&patch.path.as_path()));
        println!("{:?}", patch.checksum);
        eprintln!(
            "Patch name {}, path: {:?}, checksum: {:?}",
            &patch.name,
            &patch.path,
            &patch.checksum.to_owned().unwrap()
        );
    }
}

fn find_algorithm(srcinfo: &String) -> Result<Algorithm, String> {
    for mut line in srcinfo.lines() {
        line = line.trim();

        choose_algo!(line,
            Md5::digest, "md5",
            Sha1::digest, "sha1",
            Sha224::digest, "sha224",
            Sha256::digest, "sha256",
            Sha384::digest, "sha384",
            Sha512::digest, "sha512"
            );
    }
    Err("No algorithm found".to_string())
}

fn patch_pkgbuild(patches: &mut Vec<PatchFile>) {
    match patches
        .iter()
        .position(|item| item.name.eq_ignore_ascii_case("PKGBUILD"))
    {
        Some(i) => {
            match cmd("patch", vec!["-i", &patches[i].path.to_string_lossy()])
                .stderr_to_stdout()
                .run()
                .map_err(|e| e.to_string())
            {
                Ok(_) => {
                    eprintln!("PKGBUILD patched successfully.");
                }
                Err(error) => eprintln!("Failed to patch PKGBUILD: {}", error),
            };
            patches.remove(i);
        }
        None => {
            eprintln!("No patches for PKGBUILD found, continuing.");
        }
    }
}

fn append_patches(
    patches: &Vec<PatchFile>,
    algorithm: &Algorithm,
    srcinfo: &String,
    pkgbuild_path: &PathBuf,
) {
    let mut new_sources: Vec<String> = vec![];
    {
        let re = Regex::new(r"(?mi)(?:^\s*source = )(.*)+").unwrap();
        let sources = re.captures_iter(&srcinfo);
        for source in sources {
            new_sources.push(source[1].to_string());
        }
        for patch in patches {
            let mut dest = pkgbuild_path.to_owned();
            dest.pop();
            dest.push(&patch.path.file_name().unwrap());
            copy(&patch.path, dest).unwrap();
            new_sources.push(
                patch
                    .path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    let checksum = format!("{}sums", &algorithm.name);
    let mut new_checksums: Vec<String> = vec![];
    {
        let re = Regex::new(&(r"(?mi)(?:^\s*".to_owned() + &checksum + " = )(.*)+")).unwrap();
        let checksums = re.captures_iter(&srcinfo);
        for mut sum in checksums {
            new_checksums.push(sum[1].to_string());
        }
    }
    for mut patch in patches {
        new_checksums.push(patch.checksum.to_owned().unwrap());
    }

    let mut updpkgbuild = read_to_string(&pkgbuild_path).unwrap();

    updpkgbuild = replace_array_string(&updpkgbuild, "source".to_string(), &new_sources);
    updpkgbuild = replace_array_string(&updpkgbuild, checksum.to_string(), &new_checksums);
    updpkgbuild = prepend_prepare_patches(updpkgbuild, &patches);

    eprintln!("Writing updated PKGBUILD...");
    write!(
        &OpenOptions::new()
            .read(false)
            .write(true)
            .append(false)
            .open(&pkgbuild_path)
            .unwrap(),
        "{}",
        updpkgbuild
    );
    eprintln!("PKGBUILD written");
}

fn replace_array_string(text: &String, name: String, new_array: &Vec<String>) -> String {
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

fn prepend_prepare_patches(pkgbuild: String, patches: &Vec<PatchFile>) -> String {
    let mut prepare_start = 0;
    let mut build_start = 0;
    for (number, line) in pkgbuild.lines().enumerate() {
        if prepare_start == 0 {
            if line.starts_with("prepare() {") {
                prepare_start = number + 1;
                break;
            }
        }
        if build_start == 0 {
            if line.starts_with("build() {") {
                build_start = number + 1;
            }
        }
    }

    let mut insert_string = "".to_string();
    let mut new_pkgbuild: Vec<String> = pkgbuild.lines().map(|s| s.to_string()).collect();
    let mut start_line = 0;
    if prepare_start != 0 {
        start_line = prepare_start;
    } else if build_start != 0 {
        start_line = build_start - 1;
        insert_string.push_str("prepare() {\n");
    }

    for patch in patches {
        insert_string.push_str(&format!(
            "\tpatch -Np1 -i ${{srcdir}}/{}\n",
            patch.path.file_name().unwrap().to_string_lossy()
        ));
    }
    if prepare_start == 0 {
        insert_string.push_str("}\n");
    }

    new_pkgbuild.insert(start_line, insert_string);

    new_pkgbuild.join("\n")
}
