use std::fs::File;
use std::io::prelude::*;

pub fn file_to_string(mut file: File) -> (Result<String, String>) {
    let mut file_contents = String::new();
    // Read file to string and return error if fails
    match file
        .read_to_string(&mut file_contents)
        .map_err(|e| e.to_string())
    {
        Ok(_) => Ok(file_contents),
        Err(error) => Err(error),
    }
}