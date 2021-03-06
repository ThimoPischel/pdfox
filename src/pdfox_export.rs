use crate::pdfox_factory::*;
use crate::pdfox_link::*;
use crate::pdfox_layout::*;
use crate::pdfox_prefab::*;
use crate::pdfox_object::*;
use std::collections::HashMap;
use serde_json::Value;
use printpdf::PdfDocumentReference;
use printpdf::IndirectFontRef;

pub struct PdfoxExport {
    pub export_name: String,
    pub links: HashMap<String, PdfoxObject>,
    pub prefabs: Vec<PdfoxPrefab>
}
impl PdfoxExport {
    pub fn from_json(json: &Value) -> Result< Vec<PdfoxExport>, Vec<String> > {
        let mut result : Vec<PdfoxExport> = Vec::new();
        
        let json_array = match json.as_array() {
            None => return Err(vec!["Export (data.json) has to be an array".to_string()]),
            Some(ok) => ok
        };


        let mut count = 0;
        for array_item in json_array {
            count += 1;
            
            let single_export = match array_item.as_object() {
                Some(o) => o,
                None => return Err(vec![format!("the {} export array item is no object", count )] )
            };
            
            let export_name = match single_export["export_name"].as_str() {
                Some(o) => o.to_string(),
                None => return Err(vec![format!("the {} export array item has no 'export_name'", count)])
            };

            let links = match load_links(&single_export["links"]) {
                Ok(ok) => ok,
                Err(mut e) => {
                    e.push(format!("in the {} export with name: {}", count, export_name));
                    return Err(e);
                }
            };

            let prefabs = match PdfoxPrefab::from_json(&single_export["prefabs"]) {
                Ok(ok) => ok,
                Err(mut e) => {
                    e.push(format!("in the {} export with name: {}", count, export_name));
                    return Err(e);
                }
            };

            result.push( PdfoxExport {
                export_name,
                links,
                prefabs
            });
        }

        Ok(result)
    }

    pub fn build(&self, factory: &mut PdfoxFactory) -> Result<(),Vec<String>> 
    {
        for prefab in &self.prefabs {
            prefab.build(&mut factory);
        }

        Ok(())
    }

}



fn load_links(json: &Value) -> Result<HashMap<String, PdfoxObject>, Vec<String>> {
    let mut result : HashMap<String, PdfoxObject> = HashMap::new();

    let json_array = match json.as_array() {
        Some(o) => o,
        None => return Err(vec!["link map is no array".to_string()])
    };

    let mut count = 0;
    for array_item in json_array {
        count += 1;
        let json_object = match array_item.as_object() {
            Some(o) => o,
            None => return Err(vec![format!("the {} link map item is no object", count)])
        };
        let linkname = match &json_object["link"] {
            Value::String(o) => o.to_string(),
            _ => return Err(vec![format!("the {} link map item has no 'link' string field", count)])
        };
        let object = match PdfoxObject::from_json(&array_item) {
            Err(mut e) => {
                e.push("while loading links".to_string());
                return Err(e);
            }
            Ok(ok) => match ok {
                Some(o) => o,
                None => return Err(vec!["a link is not parsable".to_string()])
            }
        };
        result.insert(linkname, object);
    }

    Ok(result)
}
