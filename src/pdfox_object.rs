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
    pub fn from_json(json: &Value) -> Result<Option<PdfoxObject>, Vec<String>>{
        let json_object = match json {
            Value::Object(o) => o,
            _ => return Ok(None)
        };

        let pdfox_object_type = match &json_object["type"] {
            Value::String(s) => s,
            _ => return Err(vec!["'type' is not set or string".to_string()])
        };

        let pdfox_object = &json_object["value"];
        if pdfox_object.is_null() {
            return Err(vec!["'value' is not set".to_string()])
        }

        Ok(Some( match pdfox_object_type.as_str() {
            "link" => match pdfox_object {
                Value::String(x) => PdfoxObject::Link(x.to_string()),
                _ => return Err(vec!["Type 'link', but value is no String".to_string()])
            },
            
            "float" => match pdfox_object {
                Value::Number(x) => PdfoxObject::Float(
                    match x.as_f64() {
                        Some(n) => n,
                        None => return Err(vec!["type was 'float', but the number is no float".to_string()])
                    }),
                _ => return Err(vec!["Type 'float', but value is no float number".to_string()])
            },

            "int" => match pdfox_object {
                Value::Number(x) => PdfoxObject::Int(
                    match x.as_i64() {
                        Some(n) => n,
                        None => return Err(vec!["type was 'int', but the number is no integer".to_string()])
                    }),
                _ => return Err(vec!["Type 'int', but value is no integer number".to_string()])
            },

            "bool" => match pdfox_object {
                Value::Bool(x) => PdfoxObject::Bool(x.clone()),
                _ => return Err(vec!["Type 'bool', but value is no bool".to_string()])
            },

            "string" => match pdfox_object {
                Value::String(x) => PdfoxObject::String(x.clone()),
                _ => return Err(vec!["Type 'string', but value is no String".to_string()])
            },

            "line" => PdfoxObject::Line(
                match PdfoxLine::from_json(&pdfox_object){
                    Ok(o) => o,
                    Err(mut e) => {
                        e.push("in object".to_string());
                        return Err(e)
                    }
                }),

            "color" => PdfoxObject::Color(
                match PdfoxColor::from_json(&pdfox_object) {
                    Ok(o) => o,
                    Err(mut e) => {
                        e.push("in object".to_string());
                        return Err(e)
                    }
                }),
            "points" => PdfoxObject::Points(
                match PdfoxPoints::from_json(&pdfox_object) {
                    Ok(o) => o,
                    Err(mut e) => {
                        e.push("in object".to_string());
                        return Err(e)
                    }
                }),

            _ => return Err(vec!["No valid object type".to_string()])
        }))
    }
}
