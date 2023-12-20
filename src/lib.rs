struct Test {
    path: String,
    description: String
}

// @用來分模組的@
pub mod shared {
    pub mod utils;
}

pub mod commands {
    pub mod cli;
    pub use cli::Cli;
    pub mod config;
    pub use config::Configurations;
}

pub mod core {
    pub mod search_files;
    
    pub use search_files::search_files;
    pub mod extract_describe_text;
    pub use extract_describe_text::extract_describe_text;

    pub fn start(config: crate::commands::config::Configurations) {
        let current_dir = config.path.clone();
        
        let files = search_files::search_files(&config.path.clone(), config);
        let mut result: Vec<crate::Test> = Vec::new();

        files.iter().enumerate().for_each(|(idx, file)| {
            let absolute_path = std::path::PathBuf::from(file);
            let relative_path = absolute_path.strip_prefix(&current_dir).unwrap_or(&absolute_path);
            
            result.push(crate::Test { 
                path: relative_path.display().to_string(), 
                description: self::extract_describe_text(&file).unwrap() 
            });
            // println!("{}: {}", relative_path.display(), self::extract_describe_text(&file).unwrap());

            crate::shared::utils::display_progress(idx, files.len());
        });
    }
}