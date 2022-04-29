mod pdfox_color;
mod pdfox_component;
mod pdfox_core;
mod pdfox_layout;
mod pdfox_line;
mod pdfox_point;


use printpdf::*;
use std::ops;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use serde::Deserialize;
use serde_json::{Value};
use std::collections::HashMap;

use pdfox_core::*;
use pdfox_component::*;
use pdfox_layout::*;

fn main() {
    println!("\n************Starting**************\n");
    println!("{:?}", PdfoxLayoutGroup::load_from_file(&"E:\\Code\\pdfox\\layout.json".to_string()));
}
