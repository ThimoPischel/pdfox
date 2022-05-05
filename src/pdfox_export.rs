use serde_json::Value;
use crate::pdfox_object::*;

pub struct PdfoxExport {
    export_name: String,
    links: HashMap<String, PdfoxObject>,
    objects: Vec<PdfoxObject>
}
impl PdfoxExport {
    pub fn from_json(json: &Value) -> Result< Vec<PdfoxExport>, Vec<&str> > {
        let mut result : Vec<PdfoxExport> = Vec::new();
        
        let json_array = match json.as_array() {
            Err(e) => return Err(vec!["Export (data.json) has to be an array"]),
            Ok(ok) => ok
        };


        let mut count = 0;
        for array_item in json_array {
            count += 1;
            
            let single_export = match array_item.as_object() {
                Some(o) => o,
                None => return Err(vec![format!("the {} export array item is no object", count )] )
            };
            
            let mut export_name = match single_export["export_name"].as_str() {
                Some(o) => o,
                None => return Err(vec![format!("the {} export array item has no 'export_name'")])
            };

            let mut links = match load_links(&single_export["links"]) {
                Ok(ok) => ok,
                Err(e) => {
                    e.push(format!("in the {} export with name: {}", count, export_name));
                    return Err(e);
                }
            };

            let mut objects = match load_objects(&single_export["objects"]) {
                Ok(ok) => ok,
                Err(e) => {
                    e.push(format!("in the {} export with name: {}", count, export_name));
                    return Err(e);
                }
            };

            result.push( PdfoxExport {
                export_name,
                links,
                objects
            });
        }

        Ok(result)
    }

    fn load_objects(json: &Value) -> Result< Vec<PdfoxObject>, Vec<&str> > {
        let mut result : Vec<PdfoxObject> = Vec::new();

        let json_array = match json.as_array() {
            Some(o) => o,
            None => return Err(vec!["objects are no array"])
        };

        let mut count = 0;
        for array_item in json_array {
            count += 1;
            result.push(
                match PdfoxObject::from_json(&array_item) {
                    Ok(ok) => ok,
                    Err(e) => {
                        e.push(format!("while parsing the {} object", count));
                        return Err(e);
                    }
                }
            )
        
        Ok(result)
    }
    
    fn load_links(json: &Value) -> Result<HashMap<String, PdfoxObject>, Vec<&str>> {
        let mut result : HashMap<String, PdfoxObject> = HashMap::new();

        let json_array = match json.as_array() {
            Some(o) => o,
            None => return Err(vec!["link map is no array"])
        };

        let mut count = 0;
        for array_item in json_array {
            count += 1;
            let json_object = match array_item.as_object() {
                Some(o) => o,
                None => return Err(vec![format!("the {} link map item is no object", count)])
            };
            let linkname = match json_object["link"] {
                Value::String(o) => o,
                _ => return Err(vec![format!("the {} link map item has no 'link' string field", count)])
            };
            let object = match PdfoxObject::from_json(&array_item) {
                Err(e) => return Err(vec![e, format!("while parsing the {} link map item", count)]),
                Ok(o) => o,         
            };
            result.insert(linkname, object);
        }

        Ok(result)
    }
}
