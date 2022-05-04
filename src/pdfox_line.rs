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
        pub fn from_json(json: &Value ) -> PdfoxLine {
            let json_object = json.as_object().expect("line value is no Object!");

            if !json_object.contains_key("points"){
                panic!("A line is missing 'points'");
            }
            let points : Link<PdfoxPoints> = match PdfoxObject::from_json(&json["points"]).expect("failed parsing line, because points field") {
                PdfoxObject::Points(v) => Link::Value(v),
                PdfoxObject::Link(l) => Link::Link(l),
                _ => panic!("Line expected 'points' to be of type points!")
            };

            let is_closed = if !json_object.contains_key("is_closed"){
                Link::Value(false)
            } else {
                 match PdfoxObject::from_json(&json["is_closed"]).expect("failed parsing line, because is_closed field") {
                    PdfoxObject::Bool(v) => Link::Value(v),
                    PdfoxObject::Link(l) => Link::Link(l),
                    _ => panic!("Line expected 'is_closed' to be of type bool!")
                }
            };

            let has_fill = if !json_object.contains_key("has_fill"){
                Link::Value(false)
            } else {
                match PdfoxObject::from_json(&json["has_fill"]).expect("failed parsing line, because has_fill field") {
                    PdfoxObject::Bool(v) => Link::Value(v),
                    PdfoxObject::Link(l) => Link::Link(l),
                    _ => panic!("Line expected 'has_fill' to be of type bool!")
                }
            };
            
            let has_stroke = if !json_object.contains_key("has_stroke"){
                Link::Value(false)
            } else {
                match PdfoxObject::from_json(&json["has_stroke"]).expect("failed parsing line, because has_stroke field") {
                    PdfoxObject::Bool(v) => Link::Value(v),
                    PdfoxObject::Link(l) => Link::Link(l),
                    _ => panic!("Line expected 'has_stroke' to be of type bool!")
                }
            };

            let is_clipping_path = if !json_object.contains_key("is_clipping_path"){
                Link::Value(false)
            } else {
                match PdfoxObject::from_json(&json["is_clipping_path"]).expect("failed parsing line, because is_clipping_path field") {
                    PdfoxObject::Bool(v) => Link::Value(v),
                    PdfoxObject::Link(l) => Link::Link(l),
                    _ => panic!("Line expected 'is_clipping_path' to be of type bool!")
                }
            };

            let fill_color = if !json_object.contains_key("fill_color"){
                Link::Value(PdfoxColor::transparent())
            } else {
                match PdfoxObject::from_json(&json["fill_color"]).expect("failed parsing line, because fill_color field") {
                    PdfoxObject::Color(v) => Link::Value(v),
                    PdfoxObject::Link(l) => Link::Link(l),
                    _ => panic!("Line expected 'fill_color' to be of type color!")
                }
            };

            let border_color =  if !json_object.contains_key("border_color"){
                Link::Value(PdfoxColor::black())
            } else {
               match PdfoxObject::from_json(&json["border_color"]).expect("failed parsing line, because border_color field") {
                    PdfoxObject::Color(v) => Link::Value(v),
                    PdfoxObject::Link(l) => Link::Link(l),
                    _ => panic!("Line expected 'border_color' to be of type color!")
                }
            };

            PdfoxLine {
                points,
                is_closed,
                has_fill,
                has_stroke,
                is_clipping_path,
                fill_color,
                border_color
            }

        }
}
