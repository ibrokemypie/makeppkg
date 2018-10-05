use md5;
use sha1::{Digest, Sha1};
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fs::read;
use std::path::PathBuf;

pub fn compute_sha1(path: &PathBuf) -> String {
    let file = read(path).unwrap();
    Sha1::digest(&file)[0].to_string()
}

pub fn compute_sha224(path: &PathBuf) -> String {
    let file = read(path).unwrap();
    Sha224::digest(&file)[0].to_string()
}

pub fn compute_sha384(path: &PathBuf) -> String {
    let file = read(path).unwrap();
    Sha384::digest(&file)[0].to_string()
}

pub fn compute_sha256(path: &PathBuf) -> String {
    let file = read(path).unwrap();
    Sha256::digest(&file)[0].to_string()
}

pub fn compute_sha512(path: &PathBuf) -> String {
    let file = read(path).unwrap();
    Sha512::digest(&file)[0].to_string()
}

pub fn compute_md5(path: &PathBuf) -> String {
    let file = read(path).unwrap();
    md5::compute(&file)[0].to_string()
}
