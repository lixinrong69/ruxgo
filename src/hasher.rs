use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::utils::{log, LogLevel};
use std::collections::HashMap;
use md5;

/// Compute the hash of a vector of bytes
fn hash_u8(v: &[u8]) -> String {
    let digest = md5::compute(v);
    return format!("{:x}", digest);
}

/// Compute the MD5 hash of a file specified by path
fn hash_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    return hash_u8(&contents);
}

/// Get the hash value of a file specified by path from the path_hash HashMap
pub fn get_hash(path: &str, path_hash: &HashMap<String, String>) -> Option<String> {
    if path_hash.contains_key(path) {
        return Some(path_hash.get(path).unwrap().to_string());
    }
    return None;
}

/// Load the file paths and their corresponding hash values from a file specified by path
pub fn load_hashes_from_file(path: &str) -> HashMap<String, String> {
    let mut path_hash: HashMap<String, String> = HashMap::new();
    let path = Path::new(path);
    if !path.exists() {
        return path_hash;
    }
    let mut file = OpenOptions::new().read(true).open(path).unwrap(); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split(" ");
        let path = split.next().unwrap();
        let hash = split.next().unwrap();
        path_hash.insert(path.to_string(), hash.to_string());
    }
    return path_hash;
}

/// Save the file paths and their corresponding hash values to a file specified by path
pub fn save_hashes_to_file(path: &str, path_hash: &HashMap<String, String>) {
    let mut file = OpenOptions::new().write(true).create(true).open(path).unwrap();
    for (path, hash) in path_hash {
        let line = format!("{} {}\n", path, hash);
        file.write(line.as_bytes()).unwrap();
    }
    log(LogLevel::Debug, &format!("Saved hashes to file: {:?}", path_hash));
}

/// Check if a file specified by path has changed
pub fn is_file_changed(path: &str, path_hash: &HashMap<String, String>) -> bool {
    let hash = get_hash(path, path_hash);
    if hash.is_none() {
        return true;
    }
    let hash = hash.unwrap();
    let new_hash = hash_file(path);
    let result = hash != new_hash;
    if result {
        log(LogLevel::Debug, &format!("File changed, updating hash for file: {}", path));
        log(LogLevel::Debug, &format!("\tOld hash: {}", hash));
        log(LogLevel::Debug, &format!("\tNew hash: {}", new_hash));
    }
    result
}

/// Compute the hash of a file specified by path and update its value in the path_hash HashMap
pub fn save_hash(path: &str, path_hash: &mut HashMap<String, String>) {
    let new_hash = hash_file(path);
    let hash = get_hash(path, path_hash);
    if hash.is_none() {
        path_hash.insert(path.to_string(), new_hash);
        return;
    }
    let hash = hash.unwrap();
    if hash != new_hash {
        log(LogLevel::Info, &format!("File changed, updating hash for file: {}", path));
        path_hash.insert(path.to_string(), new_hash);
        return;
    }
}