use printpdf::*;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use serde::Deserialize;
use serde_json::{Value};
use std::collections::HashMap;


struct LayoutGroup {
    layouts: HashMap<String,Layout>
}

struct Layout {
    size_x:  f64,
    size_y:  f64,
    objects: Vec<PdfoxObj>
}

enum PdfoxObj {
    Link(String),
    Color(Color),
    String(String),
    Line(Box<PdfoxLine>),
    Points(Points),
    Point(Point)
}

trait PdfoxComponent {
    fn from_json(mut &self, serde_json::HashMap<String, Value>);
}

struct V2 {
    x: f64,
    y: f64
}

struct Point {
    is_relative: bool,
    position: V2
}
struct Points {
    is_relative: bool,
    positions: Vec<V2>
}
struct PdfoxLine {
    points:           PdfoxObj,
    is_closed:        PdfoxObj,
    has_fill:         PdfoxObj,
    has_stroke:       PdfoxObj,
    is_clipping_path: PdfoxObj,
    fill_color:       PdfoxObj,
    border_color:     PdfoxObj
}

impl PdfoxComponent for PdfoxLine {
    fn from_json(mut &self, serde_json::HashMap<String, Value>) {

    }
}

fn main() {
    
}

fn build_layout(layout_path: &String) -> LayoutGroup {
    let mut file = File::open(&layout_path).expect("File could not be opened");
    let mut buf = BufReader::new(file);
    let json: Value = serde_json::from_reader(buf).expect("Invalid json format");
    let mut lg = LayoutGroup::new();

    let emsg = "no matching layout format";

    for j_lay in json.as_array().expect(emsg) {
        let mut tmp_lay : Layout = Layout::new();
        let j_lay = j_lay.as_object().expect(emsg);

        //get header inforamtion
        tmp_lay.size_x = j_lay["size_x"].as_f64().expect(emsg);
        tmp_lay.size_y = j_lay["size_y"].as_f64().expect(emsg);

        //get objects
        for j_obj in j_lay["objects"].as_array().expect(emsg) {
            let j_obj = j_obj.as_object().expect(emsg);
            tmp_lay.objects.push( json_obj_to_pdfox(j_obj) );
        }
    }
    return LayoutGroup::new();
}

fn json_obj_to_pdfox (j_obj : &serde_json::Map<String,Value> ) -> PdfoxObj {
    
}
