use crate::pdfox_core::*;
use crate::pdfox_color::*;
use crate::pdfox_point::*;
use serde_json::Value;
use crate::pdfox_component::*;

#[derive(Debug)]
pub struct PdfoxLine {
    points:           Linkable<PdfoxPoints>,
    is_closed:        Linkable<bool>,
    has_fill:         Linkable<bool>,
    has_stroke:       Linkable<bool>,
    is_clipping_path: Linkable<bool>,
    fill_color:       Linkable<PdfoxColor>,
    border_color:     Linkable<PdfoxColor>
}

impl PdfoxLine {
        pub fn new(json: &Value ) -> PdfoxLine {
            let json_object = json.as_object().expect("line value is no Object!");

            if !json_object.contains_key("points"){
                panic!("A line is missing 'points'");
            }
            let points : Linkable<PdfoxPoints> = match json_obj_to_pdfox(&json["points"]) {
                PdfoxComponent::Points(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'points' to be of type points!")
            };

            if !json_object.contains_key("is_closed"){
                panic!("A line is missing 'is_closed'");
            }
            let is_closed : Linkable<bool> = match json_obj_to_pdfox(&json["is_closed"]) {
                PdfoxComponent::Bool(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'is_closed' to be of type bool!")
            };

            if !json_object.contains_key("has_fill"){
                panic!("A line is missing 'has_fill'");
            }
            let has_fill : Linkable<bool> = match json_obj_to_pdfox(&json["has_fill"]) {
                PdfoxComponent::Bool(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'has_fill' to be of type bool!")
            };
            
            if !json_object.contains_key("has_stroke"){
                panic!("A line is missing 'has_stroke'");
            }
            let has_stroke : Linkable<bool> = match json_obj_to_pdfox(&json["has_stroke"]) {
                PdfoxComponent::Bool(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'has_stroke' to be of type bool!")
            };

            if !json_object.contains_key("is_clipping_path"){
                panic!("A line is missing 'is_clipping_path'");
            }
            let is_clipping_path : Linkable<bool> = match json_obj_to_pdfox(&json["is_clipping_path"]) {
                PdfoxComponent::Bool(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'is_clipping_path' to be of type bool!")
            };

            if !json_object.contains_key("fill_color"){
                panic!("A line is missing 'fill_color'");
            }
            let fill_color : Linkable<PdfoxColor> = match json_obj_to_pdfox(&json["fill_color"]) {
                PdfoxComponent::Color(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'fill_color' to be of type color!")
            };

            if !json_object.contains_key("border_color"){
                panic!("A line is missing 'border_color'");
            }
            let border_color : Linkable<PdfoxColor> = match json_obj_to_pdfox(&json["border_color"]) {
                PdfoxComponent::Color(v) => Linkable::Value(v),
                PdfoxComponent::Link(l) => Linkable::Link(l),
                _ => panic!("Line expected 'border_color' to be of type color!")
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
