use crate::pdfox_line::*;
use crate::pdfox_core::*;
use crate::pdfox_point::*;
use crate::pdfox_color::*;

#[derive(Debug)]
pub enum PdfoxComponent {
    Link(String),
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Point(PdfoxPoint),
    Points(PdfoxPoints),
    Line(PdfoxLine),
    Color(PdfoxColor)
}
