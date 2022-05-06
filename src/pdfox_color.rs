use serde_json::Value;

#[derive(Debug)]
pub struct PdfoxColor_RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}
impl PdfoxColor_RGBA {
    pub fn from_json(json: &Value) -> Result<PdfoxColor_RGBA, Vec<String>> {
        let json_object = match json.as_object() {
            Some(o) => o,
            None => return Err(vec!["color value is no object".to_string()])
        };

        let mut color =  PdfoxColor_RGBA::black();

        color.r = match &json_object["r"] {
            Value::Number(number) => number.as_f64().expect("RGBA color r failed parsing") as f32,
            Value::Null => color.r,
            _ => return Err(vec!["rgba color 'r' have to be a number".to_string()])
        };

        color.g = match &json_object["g"] {
            Value::Number(number) => number.as_f64().expect("RGBA color g failed parsing") as f32,
            Value::Null => color.g,
            _ => return Err(vec!["rgba color 'g' have to be a number".to_string()])
        };

        color.b = match &json_object["b"] {
            Value::Number(number) => number.as_f64().expect("RGBA color b failed parsing") as f32,
            Value::Null => color.b,
            _ => return Err(vec!["rgba color 'b' have to be a number".to_string()])
        };
        
        color.a = match &json_object["a"] {
            Value::Number(number) => number.as_f64().expect("RGBA color a failed parsing") as f32,
            Value::Null => color.a,
            _ => return Err(vec!["rgba color 'a' have to be a number".to_string()])
        };

        Ok(color)
    }

    pub fn black() -> PdfoxColor_RGBA {
        PdfoxColor_RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    }

    pub fn white() -> PdfoxColor_RGBA {
        PdfoxColor_RGBA {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0
        }
    }

    pub fn transparent() -> PdfoxColor_RGBA {
        PdfoxColor_RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    }
}


#[derive(Debug)]
pub enum PdfoxColor {
    RGBA(PdfoxColor_RGBA)
}
impl PdfoxColor {
    pub fn from_json(json: &Value) -> Result<PdfoxColor, Vec<String>> {
        let json_object = match json.as_object() {
            Some(o) => o,
            None => return Err(vec!["color value is no object".to_string()])
        };

        let color_type = match json_object["type"].as_str() {
            Some(o) => o,
            None => return Err(vec!["color has no type".to_string()])
        };

        match color_type {
            "rgba" => Ok(PdfoxColor::RGBA(
                    match PdfoxColor_RGBA::from_json(&json) {
                        Ok(o) => o,
                        Err(mut e) => {
                            e.push("in color object".to_string());
                            return Err(e);
                        }})),
            _ => return Err(vec!["unsoported color type".to_string()])
        }
    }

    pub fn black() -> PdfoxColor {
        PdfoxColor::RGBA(PdfoxColor_RGBA::black())
    }
    pub fn white() -> PdfoxColor {
        PdfoxColor::RGBA(PdfoxColor_RGBA::white())
    }
    pub fn transparent() -> PdfoxColor {
        PdfoxColor::RGBA(PdfoxColor_RGBA::transparent())
    }
}
