mod pdfox_color;
mod pdfox_object;
mod pdfox_line;
mod pdfox_points;
mod pdfox_link;
mod pdfox_layout;
mod pdfox_pagegroup;
mod pdfox_page;
mod args;

use args::*;

fn main() {
    println!("\n************Starting**************\n");
    let my_args = Args::new();
    println!("{:?}", my_args);
}
