use serde_json::Value;
use std::collections::HashMap;
use crate::pdfox_pagegroup::*;


pub enum PdfoxLayout {
    Pagegroup(PdfoxPagegroup)
}

impl PdfoxLayout {
    pub fn from_json(json: &Value) -> Result<HashMap<String, PdfoxLayout>, Vec<String>> {
        let mut layouts : HashMap<String, PdfoxLayout> = HashMap::new();
        let json_array = match json.as_array() {
            Some(o) => o,
            None => return Err(vec!["Layout json has to be an array".to_string()])
        };

        for array_item in json_array {
            let layout_object = match array_item.as_object() {
                Some(o) => o,
                None => return Err(vec!["Layoutarry have to be of Objects".to_string()])
            };
            let layout_type = match layout_object["type"].as_str() {
                Some(o) => o,
                None => return Err(vec!["layout type has to be a string".to_string()])
            };
            let layout_name = match layout_object["name"].as_str() {
                Some(o) => o,
                None => return Err(vec!["layout name has to be a string".to_string()])
            };
            let layout = match layout_name {
                "pagegroup" => PdfoxLayout::Pagegroup( 
                    match PdfoxPagegroup::from_json(&layout_object["value"]) {
                        Ok(ok) => ok,
                        Err(mut e) => {
                            e.push("in layout".to_string());
                            return Err(e);
                        }
                    }),
                _ => return Err(vec!["unsupported layout type".to_string()])
            };

            layouts.insert(layout_name.to_string(), layout);
        }

        Ok(layouts)
    }
}
