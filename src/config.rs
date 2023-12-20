use std::path::PathBuf;

// @解析使用者參數過後的設定參數@
pub struct Configurations {
    pub path: PathBuf,
    pub recursive: bool,
    pub extentions: Vec<String>,
    pub output: Option<PathBuf>,
}