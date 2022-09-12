use crate::pdfox_factory::*;
use crate::pdfox_pagegroup::*;
use crate::pdfox_layout::*;
use crate::pdfox_prefab::*;
use serde_json::Value;

pub enum PdfoxPrefab {
    Layout(String),
    Pagegroup(PdfoxPagegroup)
}
impl PdfoxPrefab {
    pub fn from_json(json: &Value) -> Result<Vec<PdfoxPrefab>, Vec<String>> {
        let mut result : Vec<PdfoxPrefab> = Vec::new();

        let json_array = match json.as_array() {
            Some(o) => o,
            None => return Err(vec!["Prefabs has to be an array".to_string()])
        };

        for array_item in json_array {
            let prefab_object = match array_item.as_object() {
                Some(o) => o,
                None => return Err(vec!["Prefabarray has to be of Objects".to_string()])
            };

            let prefab_type = match prefab_object["type"].as_str() {
                Some(o) => o,
                None => return Err(vec!["Prefab type has to be a string".to_string()])
            };

            let prefab = match prefab_type {

                "layout" => PdfoxPrefab::Layout(
                    match prefab_object["value"].as_str() {
                        Some(ok) => ok.to_string(),
                        None => return Err(vec!["prefab value have to be a string if the type is 'layout'".to_string()])
                    }),

                "pagegroup" => PdfoxPrefab::Pagegroup( 
                    match PdfoxPagegroup::from_json(&prefab_object["value"]) {
                        Ok(ok) => ok,
                        Err(mut e) => {
                            e.push("in prefab pagegroup".to_string());
                            return Err(e);
                        }
                    }),

                _ => return Err(vec!["unsupported prefab type".to_string()])
            };
            result.push(prefab);
        }

        Ok(result)
    }

    pub fn build(&self, factory: &mut PdfoxFactory)
        -> Result<(),Vec<String>> 
    {
        match &self {
            PdfoxPrefab::Layout(s) => match factory.layouts.layout_map.get(s) {
                Some(layout) => match layout.build(&mut factory){
                    Err(mut e) => {
                        e.push(format!("while building layout {}", s));
                        return Err(e);
                    }
                },
                None => return Err(vec!["try to use a layout without matching link name".to_string()])
            }
            PdfoxPrefab::Pagegroup(p) => p.build(&mut factory)
        };

        Ok(())
    }
}

