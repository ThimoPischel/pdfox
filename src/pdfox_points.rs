use serde_json::Value;
use std::ops;


#[derive(Debug)]
pub struct PdfoxPoints {
    is_relative: bool,
    koords: Vec<PdfoxV2>
}
impl PdfoxPoints {
    pub fn from_json(json: &Value) -> PdfoxPoints {
        let json_object = json.as_object().expect("Points value is no object!");
        let mut points = PdfoxPoints {
            is_relative: false,
            koords: Vec::new()
        };

        let is_relative = json_object["is_relative"];
        points.is_relative = match is_relative {
            Value::Null => points.is_relative,
            Value::Bool(b) => b,
            _ => panic!("points 'is_relative' field should be a bool")
        };

        for point in json_object["koords"].as_array().expect("koords has to be an array") {
            points.koords.push(PdfoxV2::new(&point));
        }

        points
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
