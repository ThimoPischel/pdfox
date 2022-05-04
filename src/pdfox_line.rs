use crate::pdfox_link::*;
use crate::pdfox_color::*;
use crate::pdfox_points::*;
use crate::pdfox_object::*;
use serde_json::Value;

#[derive(Debug)]
pub struct PdfoxLine {
    points:           Link<PdfoxPoints>,
    is_closed:        Link<bool>,
    has_fill:         Link<bool>,
    has_stroke:       Link<bool>,
    is_clipping_path: Link<bool>,
    fill_color:       Link<PdfoxColor>,
    border_color:     Link<PdfoxColor>
}

impl PdfoxLine {
        pub fn from_json(json: &Value ) -> Result<PdfoxLine, Vec<&str>> {
            let json_object = json.as_object().expect("line value is no Object!");

            if !json_object.contains_key("points"){
                panic!("A line is missing 'points'");
            }

            let points : Link<PdfoxPoints> = match PdfoxObject::from_json(&json["points"]) {
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok { 
                    None => return Err(vec!["points are required in line"]),
                    Some(o) => match o {
                        PdfoxObject::Points(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => return Err(vec!["Line expected 'points' to be of type points!"]) 
                    }
                }
            };

            let is_closed = match PdfoxObject::from_json(&json["is_closed"]) {
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok {
                    None => Link::Value(false),
                    Some(o) => match o {
                        PdfoxObject::Bool(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => return Err(vec!["Line expected 'is_closed' to be of type bool!"])
                    }
                }
            };

            let has_fill = match PdfoxObject::from_json(&json["has_fill"]){
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok {
                    None => Link::Value(false),
                    Some(o) => match o {
                        PdfoxObject::Bool(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => return Err(vec!["Line expected 'has_fill' to be of type bool!"])
                    }
                }
            };
            
            let has_stroke = match PdfoxObject::from_json(&json["has_stroke"]){
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok {
                    None => Link::Value(false),
                    Some(o) => match o {
                        PdfoxObject::Bool(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => panic!("Line expected 'has_stroke' to be of type bool!")
                    }
                }
            };

            let is_clipping_path = match PdfoxObject::from_json(&json["is_clipping_path"]){
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok {
                    None => Link::Value(false),
                    Some(o) => match o {
                        PdfoxObject::Bool(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => return Err(vec!["Line expected 'is_clipping_path' to be of type bool!"])
                    }
                }
            };

            let fill_color = match PdfoxObject::from_json(&json["fill_color"]) {
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok {
                    None => Link::Value(PdfoxColor::black()),
                    Some(o) => match o {
                        PdfoxObject::Color(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => return Err(vec!["Line expected 'fill_color' to be of type color!"])
                    }
                }
            };

            let border_color = match PdfoxObject::from_json(&json["border_color"]) {
                Err(e) => {
                    e.push("in line");
                    return Err(e);
                },
                Ok(ok) => match ok {
                    None => Link::Value(PdfoxColor::black()),
                    Some(o) => match o {
                        PdfoxObject::Color(v) => Link::Value(v),
                        PdfoxObject::Link(l) => Link::Link(l),
                        _ => panic!("Line expected 'border_color' to be of type color!")
                    }
                }
            };

            Ok( PdfoxLine {
                points,
                is_closed,
                has_fill,
                has_stroke,
                is_clipping_path,
                fill_color,
                border_color
            })

        }
}
