use serde_json::Value;
use std::collections::HashMap;
use crate::pdfox_pagegroup::*;
use crate::pdfox_prefab::*;


pub struct PdfoxLayout {
    pub layouts: HashMap<String, Vec<PdfoxPrefab>>
}
impl PdfoxLayout {
    pub fn new() -> PdfoxLayout { PdfoxLayout { layouts: HashMap::new() }}

    pub fn from_json(json: &Value) -> Result<PdfoxLayout, Vec<String>> {
        let mut layouts : HashMap<String, Vec<PdfoxPrefab>> = HashMap::new();
        let json_array = match json.as_array() {
            Some(o) => o,
            None => return Err(vec!["Layout json has to be an array".to_string()])
        };

        for array_item in json_array {
            let layout_object = match array_item.as_object() {
                Some(o) => o,
                None => return Err(vec!["Layoutarray has to be of objects".to_string()])
            };
            
            let layout_name = match layout_object["name"].as_str() {
                Some(o) => o,
                None => return Err(vec!["layout name has to be a string".to_string()])
            };

            let prefabs = match PdfoxPrefab::from_json( &layout_object["prefabs"] ) {
                Ok(ok) => ok,
                Err(mut e) => {
                    e.push("in layout".to_string());
                    return Err(e);
                }
            };

            layouts.insert(layout_name.to_string(), prefabs);
        }

        Ok(PdfoxLayout { layouts })
    }
}
