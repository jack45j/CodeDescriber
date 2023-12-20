use std::path::PathBuf;
use xlsxwriter::{Workbook, XlsxError};

// @處理資料且導出為xlsx的服務@
pub struct CodeDescription {
    pub path: String,
    pub description: String
}

impl CodeDescription {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            description: String::new()
        }
    }
}

pub struct XlsxService {
    code_descriptions: Vec<CodeDescription>,
}

impl XlsxService {
    pub fn new(descriptions: Vec<CodeDescription>) -> Self {
        Self {
            code_descriptions: descriptions
        }
    }

    pub fn push(&mut self, description: CodeDescription) {
        self.code_descriptions.push(description);
    }

    pub fn export(&self, path: PathBuf) -> std::result::Result<(), XlsxError> {
        let xlsx = Workbook::new(path.to_str().unwrap())?;
        let mut worksheet = xlsx.add_worksheet(None)?;

        for (idx, item) in self.code_descriptions.iter().enumerate() {
            worksheet.write_string(idx as u32, 0, &item.path, None)?;
            worksheet.write_string(idx as u32, 1, &item.description, None)?;
        }

        xlsx.close()?;
        Result::Ok(())
    }
}