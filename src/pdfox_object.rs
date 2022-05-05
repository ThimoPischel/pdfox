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
    pub fn from_json(json: &Value) -> Result<Option<PdfoxObject>, Vec<&str>>{
        let json_object = match json {
            Value::Object(o) => o,
            _ => return Ok(None)
        };

        let pdfox_object_type = match &json_object["type"] {
            Value::String(s) => s,
            _ => return Err(vec!["'type' is not set or string"])
        };

        let pdfox_object = &json_object["value"];
        if pdfox_object.is_null() {
            return Err(vec!["'value' is not set"])
        }

        Ok(Some( match pdfox_object_type.as_str() {
            "link" => match pdfox_object {
                Value::String(x) => PdfoxObject::Link(x),
                _ => return Err(vec!["Type 'link', but value is no String"])
            },
            
            "float" => match pdfox_object {
                Value::Number(x) => PdfoxObject::Float(
                    match x.as_f64() {
                        Some(n) => n,
                        None => return Err(vec!["type was 'float', but the number is no float"])
                    }),
                _ => return Err(vec!["Type 'float', but value is no float number"])
            },

            "int" => match pdfox_object {
                Value::Number(x) => PdfoxObject::Int(
                    match x.as_i64() {
                        Some(n) => n,
                        None => return Err(vec!["type was 'int', but the number is no integer"])
                    }),
                _ => return Err(vec!["Type 'int', but value is no integer number"])
            },

            "bool" => match pdfox_object {
                Value::Bool(x) => PdfoxObject::Bool(x),
                _ => return Err(vec!["Type 'bool', but value is no bool"])
            },

            "string" => match pdfox_object {
                Value::String(x) => PdfoxObject::String(x),
                _ => return Err(vec!["Type 'string', but value is no String"])
            },

            "line" => PdfoxObject::Line(
                match PdfoxLine::from_json(&pdfox_object){
                    Ok(o) => o,
                    Err(e) => {
                        e.push("in object");
                        return Err(e)
                    }
                }),

            "color" => PdfoxObject::Color(
                match PdfoxColor::from_json(&pdfox_object) {
                    Ok(o) => o,
                    Err(e) => {
                        e.push("in object");
                        return Err(e)
                    }
                }),
            "points" => PdfoxObject::Points(
                match PdfoxPoints::from_json(&pdfox_object) {
                    Ok(o) => o,
                    Err(e) => {
                        e.push("in object");
                        return Err(e)
                    }
                }),

            _ => return Err(vec!["No valid object type"])
        }))
    }
}
