use crate::pdfox_object::*;
use serde_json::Value;

pub struct PdfoxPage {
    size_x: f64,
    size_y: f64,
    objects: Vec<PdfoxObject>
}
impl PdfoxPage {
    pub fn from_json(json: &Value) -> Result<PdfoxPage, Vec<&str>> {
        let json_object = match json.as_object() {
            Some(o) => o,
            None => return Err(vec!["page is no object"])
        };
        let mut page = PdfoxPage {
            size_x: 297.0,
            size_y: 210.0,
            objects: Vec::new()
        };

        page.size_x = match json_object["size_x"] {
            Value::Number(n) => n.as_f64().expect("size_x and size_y of page have to be a flaot"),
            Value::Null => page.size_x,
            _ => return Err(vec!["size_x and size_y of page have to be a flaot"])
        };
        page.size_y = match json_object["size_y"] {
            Value::Number(n) => n.as_f64().expect("size_x and size_y of page have to be a flaot"),
            Value::Null => page.size_y,
            _ => return Err( vec!["size_x and size_y of page have to be a flaot"] )
        };

        let array = json_object["objects"].as_array().expect("page objects have to be an array");
        for object in array {
            page.objects.push ( 
                match PdfoxObject::from_json(&object) {
                    Err(e) => {
                        e.push("in page");
                        return Err(e);
                    }
                    Ok(ok) => match ok {
                        Some(o) => match &o {
                            PdfoxObject::Link(_) => o,
                            PdfoxObject::Line(_) => o,
                            _ => return Err(vec!["page object type not allowed (link | line)"])
                        }
                    }
                }
            );
        }
        Ok(page)
    }
}
