use serde_json::Value;
use crate::pdfox_page::*;

pub struct PdfoxPagegroup {
    pages: Vec<PdfoxPage>
}
impl PdfoxPagegroup {
    pub fn from_json(json: &Value) -> Result<PdfoxPagegroup, Vec<String>> {
        let mut result = PdfoxPagegroup {
            pages: Vec::new()
        };
        
        let json_array = match json.as_array() {
            Some(o) => o,
            None => return Err(vec!["pagegroup value is no array".to_string()])
        };

        for array_item in json_array {
            result.pages.push( match PdfoxPage::from_json(array_item) {
                Err(mut e) => {
                    e.push("in pagegroup".to_string());
                    return Err(e);
                },
                Ok(ok) => ok
            })
        };

        Ok(result)
    }
}
