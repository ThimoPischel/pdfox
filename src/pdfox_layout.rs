use serde_json::Value;
use std::collections::HashMap;
use crate::pdfox_pagegroup::*;


pub enum Layout {
    Pagegroup(PdfoxPagegroup)
}

impl Layout {
    pub fn from_json(json: &Value) -> Result<HashMap<String, Layout>, &str> {
        let mut layouts : HashMap<String, Layout> = HashMap::new();
        let json_array = match json.as_array() {
            None => return Err(vec!["Layout json has to be an array"]),
            Some(o) => o
        };

        let mut counter = 0;
        for array_item in json_array {
            counter += 1;
            let layout_object = match array_item.as_object() {
                None => return Err(vec![format!("the {} layout array item is no object", counter)]),
                Some(o) => o
            };
            let layout_name = match layout_object["name"].as_str() {
                None => return Err(vec![format!("the {}. layout name is no string", counter)]),
                Some(o) => o
            };
            let layout_type = match layout_object["type"].as_str() {
                None => return Err(vec![format!("the {} layout '{}' type has to be a string", counter, layout_name)]),
                Some(o) => o
            };
            
            let layout = match layout_name {
                "pagegroup" => Layout::Pagegroup(PdfoxPagegroup::from_json(&layout_object["value"])),
                _ => return Err(vec![format!("the {} layout '{}' has an invalid layout type", counter, layout_name)])
            };

            layouts.insert(layout_name.to_string(), layout);
        }

        Ok(layouts)
    }
}
