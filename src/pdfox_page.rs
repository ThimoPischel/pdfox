use crate::pdfox_object::*;
use serde_json::Value;

pub struct PdfoxPage {
    size_x: f64,
    size_y: f64,
    objects: Vec<PdfoxObject>
}
impl PdfoxPage {
    pub fn from_json(json: &Value) -> PdfoxPage {
        let json_object = json.as_object().expect("page have to be an object");
        let mut page = PdfoxPage {
            size_x: 297.0,
            size_y: 210.0,
            objects: Vec::new()
        };

        page.size_x = match json_object["size_x"] {
            Value::Number(n) => n.as_f64().expect("size_x and size_y of page have to be a flaot"),
            Value::Null => page.size_x,
            _ => panic!("size_x and size_y of page have to be a flaot")
        };
        page.size_y = match json_object["size_y"] {
            Value::Number(n) => n.as_f64().expect("size_x and size_y of page have to be a flaot"),
            Value::Null => page.size_y,
            _ => panic!("size_x and size_y of page have to be a flaot")
        };

        let array = json_object["objects"].as_array().expect("page objects have to be an array");
        for object in array {
            page.objects.push ( 
                match PdfoxObject::from_json(&object) {
                    Ok(o) => o,
                    Err(e) => {
                        println!("{}", e);
                        panic!("failed parsing objects in page");
                    }
                }
            );
        }
        page
    }
}
