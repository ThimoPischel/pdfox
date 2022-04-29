use crate::pdfox_core::*;
use crate::pdfox_component::*;
use std::collections::HashMap;
use serde_json::Result;
use serde_json::Value;
use std::io::BufReader;
use std::fs::File;
use std::fmt::Debug;

#[derive(Debug)]
pub struct PdfoxLayout {
    name:    String,
    size_x:  f64,
    size_y:  f64,
    objects: Vec<PdfoxComponent>
}
impl PdfoxLayout {
    pub fn new(json: &Value) -> PdfoxLayout {
            let obj = json.as_object().expect("Layout is no Object!");
            let mut objects : Vec<PdfoxComponent> = Vec::new();
            
            let json_objects = obj["objects"].as_array().expect("Layout without objects array");
            for json_object in json_objects {
                objects.push(json_obj_to_pdfox(json_object));
            }

            PdfoxLayout {
                name : obj["name"].as_str().expect("Layout has no name field!").to_string(),
                size_x: obj["size_x"].as_f64().expect("Layout has no size_x field!"),
                size_y: obj["size_y"].as_f64().expect("Layout has no size_y field!"),
                objects: objects
            }
    }
}


#[derive(Debug)]
pub struct PdfoxLayoutGroup {
    layouts: HashMap<String,PdfoxLayout>
}
impl PdfoxLayoutGroup {
    pub fn new(json: &Value) -> PdfoxLayoutGroup {
        let mut lg = PdfoxLayoutGroup { layouts: HashMap::new() };

        let arr = json.as_array().expect("Layouts have to be an array!");
        for lay_it in arr {
            let layout = PdfoxLayout::new(lay_it);
            lg.layouts.insert(layout.name.clone(), layout);
        }
        lg
    }

    pub fn load_from_file(file_path: &String) -> PdfoxLayoutGroup {
        println!("Start loading file \"{}\" as LayoutGroup", &file_path);
        let file = File::open(&file_path).expect("File couldnt be opened!");
        let buf = BufReader::new(file);
        let json = serde_json::from_reader(buf).expect("File is no valid json format!");
        PdfoxLayoutGroup::new(&json)
    }
}
