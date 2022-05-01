use serde_json::Value;
use std::collections::HashMap;
use crate::pdfox_pagegroup::*;


pub enum Layout {
    Pagegroup(PdfoxPagegroup)
}

impl Layout {
    pub fn from_json(json: &Value) -> HashMap<String, Layout> {
        let mut layouts : HashMap<String, Layout> = HashMap::new();
        let json_array = json.as_array().expect("Layout json has to be an array");
        for array_item in json_array {
            let layout_object = array_item.as_object().expect("Layoutarry have to be of Objects");
            let layout_type = layout_object["type"].as_str().expect("layout type has to be a string");
            let layout_name = layout_object["name"].as_str().expect("layout name has to be a string");
            
            let layout = match layout_name {
                "pagegroup" => Layout::Pagegroup(PdfoxPagegroup::from_json(&layout_object["value"])),
                _ => panic!("unsupported layout type")
            };

            layouts.insert(layout_name.to_string(), layout);
        }

        layouts
    }
}
