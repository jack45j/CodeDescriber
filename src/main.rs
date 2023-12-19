use std::{env, fs::File, ffi::OsStr, path::PathBuf, io::{self, BufRead, BufReader}};
use walkdir::WalkDir;
use dirs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let path = env::current_dir().unwrap();
    let home_dir = dirs::home_dir().unwrap();
    let proj_path: PathBuf = [home_dir.clone(), "Documents/Books/Books Modules/CmsNetworking/".into()].iter().collect();
    let (files, paths) = search_path(proj_path.clone(), &"swift");
    files.iter().for_each(|x| {
        let path = PathBuf::from(x);
        let stripped_path = path.strip_prefix(&proj_path).unwrap_or(&path);
        println!("{:?} -> {}\n", stripped_path, extract_text(x).unwrap());
    });
}

fn search_path(path: PathBuf, ext: &str) -> (Vec<String>, Vec<String>) {
    let mut files: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.file_name().and_then(OsStr::to_str).map_or(false, |s| s.starts_with('.')) {
            continue;
        }

        if path.is_dir() {
            dirs.push(path.to_str().unwrap().to_string());
        } else {
            if path.extension().and_then(OsStr::to_str) == Some(ext) {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    return (files, dirs);
}

fn extract_text(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    if let Some(Ok(first_line)) = reader.lines().next() {
        let re = Regex::new(r"@(.*)@").unwrap();
        if let Some(captures) = re.captures(&first_line) {
            if let Some(matched) = captures.get(1) {
                return Ok(matched.as_str().to_string());
            }
        }
    }
    Ok(String::new())
}

fn print_help() {
    println!("Options:");
    println!("-h, --help\n\t\tPrint this help menu");
    println!("-p <Path> --path\n\t\tPath to the directory to be scanned");
    println!("-e <Extention> --extension\t\tFile extension to be searched for. ex: swift, js, py");
    println!("-r --Recursive\t\tSearch recursively in the given directory");
}
