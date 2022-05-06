use crate::args::*;
use crate::pdfox_layout::*;
use crate::pdfox_export::*;

use std::collections::HashMap;

use serde_json::Value;
use printpdf::pdf_document::*;


pub struct PdfoxFactory {
    layouts: HashMap<String, PdfoxLayout>,
    exports: Vec<PdfoxExport>
    //fonts and images in future
}
impl PdfoxFactory {
    pub fn new(arg: &Args) -> Result<PdfoxFactory, Vec<String>> {
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
                        Ok(ok_layouts) => ok_layouts
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


    pub fn build_files(&self) -> Result<Vec<String>, Vec<String>> {
        for export in self.exports {
            let mut doc : PdfDocument = PdfDocument::new
        }
    }
}

fn json_from_file(path: &String) -> Result<Value, Vec<&str>> {
    Err(vec!["not impl".to_string()])
}
