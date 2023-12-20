use std::fs::File;
use std::io::{BufReader, Read};
use std::io;
use std::path::Path;
use regex::Regex;

// @在指定的檔案路徑內尋找說明文字@
pub fn extract_describe_text(file_path: &Path) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let regex = Regex::new(r"(?s)@(.*?)@").unwrap();
    
    return Ok(regex.captures(&content)
            .and_then(|captures| captures.get(1))
            .map(|matched| matched.as_str().to_string())
            .unwrap_or(String::new()))

}