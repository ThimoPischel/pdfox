mod pdfox_color;
mod pdfox_component;
mod pdfox_core;
mod pdfox_layout;
mod pdfox_line;
mod pdfox_point;
mod args;

use pdfox_layout::*;
use args::*;

fn main() {
    println!("\n************Starting**************\n");
    let my_args = Args::new();
    println!("{:?}", my_args);
}
