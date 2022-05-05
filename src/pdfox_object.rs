use crate::pdfox_line::*; 
use crate::pdfox_color::*;
use crate::pdfox_points::*;
use serde_json::Value;

pub enum PdfoxObject {
    Link(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
    Line(PdfoxLine),
    Points(PdfoxPoints),
    Color(PdfoxColor)
}
impl PdfoxObject {
    pub fn from_json(json: &Value) -> Result<PdfoxObject, &str> {
        let json_object = match json {
            Value::Object(o) => o,
            _ => return Err("No Object")
        };

        let pdfox_object_type = match &json_object["type"] {
            Value::String(s) => s,
            _ => return Err("Type is not set or string")
        };

        let pdfox_object = &json_object["value"];
        if pdfox_object.is_null() {
            return Err("No value set")
        }

        Ok( match pdfox_object_type.as_str() {
            "link" => match pdfox_object {
                Value::String(x) => PdfoxObject::Link(x.to_string()),
                _ => return Err("Type 'link', but value is no String")
            },
            
            "float" => match pdfox_object {
                Value::Number(x) => PdfoxObject::Float(
                    match x.as_f64() {
                        Some(n) => n,
                        None => return Err("type was 'float', but the number is no float")
                    }),
                _ => return Err("Type 'float', but value is no float number")
            },

            "int" => match pdfox_object {
                Value::Number(x) => PdfoxObject::Int(
                    match x.as_i64() {
                        Some(n) => n,
                        None => return Err("type was 'int', but the number is no integer")
                    }),
                _ => return Err("Type 'int', but value is no integer number")
            },

            "bool" => match pdfox_object {
                Value::Bool(x) => PdfoxObject::Bool(x.clone()),
                _ => return Err("Type 'bool', but value is no bool")
            },

            "string" => match pdfox_object {
                Value::String(x) => PdfoxObject::String(x.to_string()),
                _ => return Err("Type 'string', but value is no String")
            },

            "line" => PdfoxObject::Line(PdfoxLine::from_json(&pdfox_object)),
            "color" => PdfoxObject::Color(PdfoxColor::from_json(&pdfox_object)),
            "points" => PdfoxObject::Points(PdfoxPoints::from_json(&pdfox_object)),

            _ => return Err("No valid object type")
        })
    }
}
