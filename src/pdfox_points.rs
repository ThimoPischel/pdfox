use serde_json::Value;
use std::ops;


#[derive(Debug)]
pub struct PdfoxPoints {
    is_relative: bool,
    koords: Vec<PdfoxV2>
}
impl PdfoxPoints {
    pub fn from_json(json: &Value) -> Result<PdfoxPoints, Vec<String>> {
        let json_object = match json.as_object() {
            Some(o) => o,
            None => return Err(vec!["color value is no object".to_string()])
        };
        let mut points = PdfoxPoints {
            is_relative: false,
            koords: Vec::new()
        };

        points.is_relative = match &json_object["is_relative"]{
            Value::Null => points.is_relative,
            Value::Bool(b) => b.clone(),
            _ => return Err(vec!["points 'is_relative' field should be a bool".to_string()])
        };

        let json_koords = match json_object["koords"].as_array() {
            Some(o) => o,
            None => return Err(vec!["koords has to be an array".to_string()]) 
        };

        for point in json_koords {
            points.koords.push(
                match PdfoxV2::new(&point) {
                    Ok(ok) => ok,
                    Err(mut e) => {
                        e.push("while parsing points".to_string());
                        return Err(e);
                    }
                }
            );
        }

        Ok(points)
    }
}

#[derive(Debug)]
pub struct PdfoxV2 {
    x: f64,
    y: f64
}
impl PdfoxV2 {
    pub fn new(json: &Value) -> Result<PdfoxV2, Vec<String>> {
        let json_object = json.as_object().expect("V2 value is no object!");
        Ok(PdfoxV2 {
            x: match json_object["x"].as_f64() {
                Some(o) => o,
                None => return Err(vec!["V2 has no 'x' float".to_string()]),
            },
            y: match json_object["y"].as_f64() { 
                Some(o) => o,
                None => return Err(vec!["V2 has no 'y' float".to_string()])
            }
        })
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
