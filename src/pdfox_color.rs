use serde_json::Value;

#[derive(Debug)]
pub struct PdfoxColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}
impl PdfoxColor {
    pub fn new(json: &Value) -> PdfoxColor {
        let json_object = json.as_object().expect("color value is no object");
        PdfoxColor {
            r: json_object["r"].as_f64().expect("color field 'r' not found or float") as f32,
            g: json_object["g"].as_f64().expect("color field 'g' not found or float") as f32,
            b: json_object["b"].as_f64().expect("color field 'b' not found or float") as f32,
            a: json_object["a"].as_f64().expect("color field 'a' not found or float") as f32
        }
    }
}
