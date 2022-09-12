use crate::args::*;
use crate::pdfox_layout::*;
use crate::pdfox_export::*;

use std::collections::HashMap;
use std::io::{BufReader, BufWriter};
use std::fs::File; 
use std::fs::read_dir;
use std::path::Path;

use serde_json::Value;
use printpdf::pdf_document::*;
use printpdf::*;

pub struct PdfoxFactory {
    pub linkable_layouts: PdfoxLayout,
    pub exports: Vec<PdfoxExport>,
    pub font_refs: HashMap<String, Option<IndirectFontRef>>,
    pub font_paths: HashMap<String, String>,
    pub doc: PdfDocumentReference
}
impl PdfoxFactory {
    pub fn new(arg: &Args) -> Result<PdfoxFactory, Vec<String>> {
        Ok( PdfoxFactory {
            layouts: match &arg.layout_json {
                None => PdfoxLayout::new(),
                Some(ok_path) => match json_from_file(&ok_path) {
                    Err(mut e) => {
                        e.push("while parsing layouts_json".to_string());
                        return Err(e);
                    },
                    Ok(ok_value) => match PdfoxLayout::from_json(&ok_value) {
                        Err(mut e) => {
                            e.push("while parsing layouts_json".to_string());
                            return Err(e);
                        },
                        Ok(ok_layouts) => ok_layouts
                    }
                }
            }, // End of layouts


            exports: match json_from_file(&arg.data_json) {
                Err(mut e) => {
                    e.push("while parsing data_json".to_string());
                    return Err(e);
                },
                Ok(ok_value) => match PdfoxExport::from_json(&ok_value) {
                    Err(mut e) => {
                        e.push("while parsing data_json".to_string());
                        return Err(e);
                    },
                    Ok(ok_exports) => ok_exports
                }
            }, // end of exports
            

            font_paths: load_fonts(arg),
            font_refs: HashMap::new(),
            doc: PdfDocument::empty("empty")

        }) //end of PdfoxFactory
    }


    pub fn build_files(self, arg: &Args) -> Result<Vec<String>, Vec<String>> {
        let printed : Vec<String> = Vec::new();
        for export in &self.exports {
            self.doc = PdfDocument::empty(&export.export_name);

            let fonts = match add_all_fonts(&mut self.doc, &self.font_paths) {
                Ok(ok) => ok,
                Err(mut e) => {
                    e.push("while adding fonts to doc".to_string());
                    return Err(e);
                }
            };
            

            match &export.build(&mut PdfoxFactory) {
                Err(mut e) => {
                    e.push("while building doc".to_string());
                    return Err(e);
                }
                Ok(_) => self.doc.save( &mut BufWriter::new( 
                    match File::create(get_path_for_pdf(arg, &export.export_name)) {
                        Ok(file) => file,
                        Err(_) => return Err(vec!["couldnt create file".to_string()])
                    }
                ))
            };
        }
        Ok(printed)
    }
}

fn json_from_file(path: &String) -> Result<Value, Vec<String>> {
    Err(vec!["not impl".to_string()])
}

fn load_fonts(arg: &Args) -> HashMap<String, String> {   
    let mut fonts : HashMap<String, String> = HashMap::new();
    let load = | folder : &Option<String> | -> () {
        match folder {
            None => return,
            Some(folder_path) => {
                let font_folder_path = Path::new(folder_path).join("fonts");
                for font_file in read_dir(font_folder_path).expect("couldnt read fonts") {
                    let font_file = font_file.expect("font file iteration failed");
                    fonts.insert(
                        font_file.path()
                            .file_name().expect("failed getting fontname")
                            .to_str().expect("couldnt parse str")
                            .to_string(),
                        font_file.path().to_str().expect("failed getting font path").to_string()
                    );
                }
            }
        }
    };
    load(&arg.layout_dir);
    load(&arg.data_dir);
    fonts
}
fn add_all_fonts(doc: &mut PdfDocumentReference, font_path_map: &HashMap<String, String>) 
    -> Result<HashMap<String, IndirectFontRef>, Vec<String>>
{
    let font_refs : HashMap<String, IndirectFontRef> = HashMap::new();
    for (font_name, font_path) in font_path_map{
        font_refs.insert(
            font_name.clone(), 
            match doc.add_external_font(BufReader::new(File::open(font_path).expect("couldnt open font file"))) {
                Ok(ok) => ok,
                Err(e) => return Err(vec!["failed adding font".to_string()])

            }
        );
    };

    Ok(font_refs)
}
fn get_path_for_pdf(arg: &Args, name: &String) -> String {
    match Path::new(&arg.output_dir).join(name.clone() + ".pdf").to_str() {
        Some(o) => o.to_string(),
        None => panic!("failed joining path")
    }
}
