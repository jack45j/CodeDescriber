use std::{ffi::OsStr, path::PathBuf};
use walkdir::WalkDir;
use crate::commands::config::Configurations;

// @在目錄內搜尋需要爬找說明文字的方法@
pub fn search_files(root: &PathBuf, config: Configurations) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    
    let depth: usize = if config.recursive { ::std::usize::MAX } else { 1 };
    for entry in WalkDir::new(root).max_depth(depth) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.file_name().and_then(OsStr::to_str).map_or(false, |s| s.starts_with('.')) {
            continue;
        }

        if !path.is_dir() {
            let file_ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
            if config.extentions.contains(&file_ext.to_string()) {
                files.push(path.to_path_buf());
            }
        }
    }

    files
}