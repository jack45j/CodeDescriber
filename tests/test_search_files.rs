mod tests {
    use code_describer::config::Configurations;
    use code_describer::service::search_files;
    use std::env::current_dir;
    use std::path::PathBuf;

    #[test]
    fn test_search_files() {
        let config = Configurations {
            path: PathBuf::from("./tests/test_data"),
            recursive: false,
            extentions: vec!["txt".to_string(), "md".to_string()],
            output: None
        };

        
        let root = &config.path;
        let files = search_files(&root, &config);
        assert_eq!(files.len(), 2);
    }
}