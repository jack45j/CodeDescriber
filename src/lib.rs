// @用來分模組的@
pub mod common {
    pub mod utils;
}

pub mod cli;
pub use cli::Cli;
pub mod config;
pub use config::Configurations;

pub mod service {
    pub mod xlsx_service;
    pub mod search_files;
    pub use search_files::search_files;
    pub mod extract_describe_text;
    pub use extract_describe_text::extract_describe_text;

    use self::xlsx_service::{CodeDescription, XlsxService};

    pub fn start(config: crate::config::Configurations) {
        let current_dir = config.path.clone();
        
        let files = search_files::search_files(&config.path.clone(), &config);
        let mut xlsx_service = XlsxService::new(Vec::<CodeDescription>::new());

        files.iter().enumerate().for_each(|(idx, file)| {
            let absolute_path = std::path::PathBuf::from(file);
            let relative_path = absolute_path.strip_prefix(&current_dir).unwrap_or(&absolute_path);
            
            xlsx_service.push(
                CodeDescription { 
                    path: relative_path.display().to_string(), 
                    description: self::extract_describe_text(&file).unwrap() 
                }
            );
            
            crate::common::utils::display_progress(idx, files.len());
        });
        println!("");
        println!("Found {} files.", &files.len());

        let output_path = &config.output.unwrap_or(current_dir).join("result.xlsx");
        match xlsx_service.export(output_path.into()) {
            Ok(_) => println!("Exported to {}", output_path.display()),
            Err(err) => panic!("Error: {}", err)
        }
    }
}