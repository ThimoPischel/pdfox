use crate::pdfox_color::*;
use crate::pdfox_point::*;
use crate::pdfox_line::*;
use crate::pdfox_component::*;
use serde_json::Value;

#[derive(Debug)]
pub enum Linkable<T> {
    Value(T),
    Link(String)
}

pub fn json_obj_to_pdfox (json: &Value) -> PdfoxComponent {
        //println!("\n{:?}\n", &json);
        let j_obj = json.as_object().expect("json_obj_to_pdfox required an object!");
        let data = &j_obj["value"];
        let data_type = j_obj["type"].as_str().expect("no valid object -> \"type\" not found");

        match data_type {
            "link"   => PdfoxComponent::Link(data.as_str().expect("type 'link' but no string in data").to_string()),
            "string" => PdfoxComponent::String(data.as_str().expect("type 'string' but no string in data").to_string()),
            "int"    => PdfoxComponent::Int(data.as_i64().expect("type 'int' but no int in data")),
            "float"  => PdfoxComponent::Float(data.as_f64().expect("type 'float' but no float in data")),
            "bool"   => PdfoxComponent::Bool(data.as_bool().expect("type 'bool' but no bool in data")),
            "point"  => PdfoxComponent::Point(PdfoxPoint::new(&data)),
            "points" => PdfoxComponent::Points(PdfoxPoints::new(&data)),
            "color"  => PdfoxComponent::Color(PdfoxColor::new(&data)),
            "line"   => PdfoxComponent::Line(PdfoxLine::new(&data)),
            _        => panic!("{} is no valid type", data_type)
        }
}
