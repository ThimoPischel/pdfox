use crate::args::*;
use crate::pdfox_layout::*;
use crate::pdfox_export::*;

use std::collections::HashMap;

use serde_json::Value;


pub struct PdfoxFactory {
    layouts: HashMap<String, PdfoxLayout>,
    exports: Vec<PdfoxExport>
    //fonts and images in future
}
impl PdfoxFactory {
    pub fn Init(arg: &Args) -> Result<PdfoxFactory, Vec<String>> {
        PdfoxFactory {

            layouts: match arg.layout_json {
                None => HashMap::new(),
                Some(ok_path) => match json_from_file(&ok_path) {
                    Err(mut e) => {
                        e.push("while parsing layouts_json");
                        return Err(e);
                    },
                    Ok(ok_value) => match PdfoxLayout::new(&ok_value) {
                        Err(mut e) => {
                            e.push("while parsing layouts_json");
                            return Err(e);
                        },
                        Ok(ok_layouts) => layouts
                    }
                }
            }, // End of layouts


            exports: match json_from_file(&arg.data_json) {
                Err(mut e) => {
                    e.push("while parsing data_json");
                    return Err(e);
                },
                Ok(ok_value) => match PdfoxExport::from_json(&ok_value) {
                    Err(mut e) => {
                        e.push("while parsing data_json");
                        return Err(e);
                    },
                    Ok(ok_exports) => ok_exports
                }
            } // end of exports


        } //end of PdfoxFactory
    }


    pub fn BuildFiles() -> Result<(), Vec<&str>> {
        
    }
}

fn json_from_file(path: &String) -> Result<Value, Vec<&str>> {
    Err(vec!["not impl".to_string()])
}
