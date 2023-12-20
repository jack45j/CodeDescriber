use std::path::PathBuf;
use clap::Parser;
use super::config;

// @定義使用者可輸入的參數結構@

#[derive(Parser, Debug)]
#[command(
    author = "Benson Lin", 
    about = "A command line tool to search for a string in a directory of files", 
    version)]
pub struct Cli {
    #[clap(
        short = 'p', 
        long, 
        help = "Path to the directory to be scanned",)]
    pub path: Option<PathBuf>,   

    #[clap(
        short = 'r', 
        long, 
        default_value = "false",
        help = "Search recursively in the given directory")]
    pub recursive: bool,

    #[clap(
        short = 'e',
        long, 
        help = "Comma-separated file extensions to be searched for. ex: swift,js,py")]
    pub extentions: String,

    #[clap(
        short = 's',
        long, 
        help = "Output the result to a file")]
        pub output: Option<PathBuf>
}

impl Cli {
    pub fn parse_cli() -> config::Configurations {
        // Get the command line arguments
        let cli = Cli::parse();

        // Get the path to the directory to be scanned
        let path = cli.path.unwrap_or_else(|| std::env::current_dir().unwrap());
        
        // Get the recursive flag
        let recursive = cli.recursive;
        
        // Get the file extensions to be searched for
        let exts = cli.extentions.split(",").map(|ext| ext.to_string()).collect();

        config::Configurations {
            path: path,
            recursive: recursive,
            extentions: exts,
            output: cli.output
        }
    }
}