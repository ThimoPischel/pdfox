use serde_json::Value;
use std::ops;

#[derive(Debug)]
pub struct PdfoxPoint {
    is_relative: bool,
    position: PdfoxV2
}
impl PdfoxPoint {
    pub fn new(json: &Value) -> PdfoxPoint {
        let json_object = json.as_object().expect("Point value is no object!");
        if !json_object.contains_key("position") {
            panic!("Point without 'position'!");
        }
        PdfoxPoint {
            is_relative: json_object["is_relative"].as_bool().expect("Point field 'is_relative' is not found or bool"),
            position: PdfoxV2::new(&json_object["position"])
        }
    }
}

#[derive(Debug)]
pub struct PdfoxPoints {
    is_relative: bool,
    positions: Vec<PdfoxV2>
}
impl PdfoxPoints {
    pub fn new(json: &Value) -> PdfoxPoints {
        let json_object = json.as_object().expect("Points value is no object!");
        let mut positions : Vec<PdfoxV2> = Vec::new();

        for json_position in json_object["positions"].as_array().expect("Points without 'positions' array") {
            positions.push(PdfoxV2::new(json_position));
        }
        PdfoxPoints {
            is_relative: json_object["is_relative"].as_bool().expect("Points field 'is_relative' is not found or bool"),
            positions: positions
        }
    }
}

#[derive(Debug)]
pub struct PdfoxV2 {
    x: f64,
    y: f64
}
impl PdfoxV2 {
    pub fn new(json: &Value) -> PdfoxV2 {
        let json_object = json.as_object().expect("V2 value is no object!");
        PdfoxV2 {
            x: json_object["x"].as_f64().expect("V2 has no 'x' float"),
            y: json_object["y"].as_f64().expect("V2 has no 'y' float")
        }
    }
}
impl ops::Add<PdfoxV2> for PdfoxV2 {
    type Output = PdfoxV2;

    fn add(self, _rhs: PdfoxV2) -> PdfoxV2 {
        PdfoxV2{
            x : self.x + _rhs.x,
            y : self.y - _rhs.y
        }
    }
}
