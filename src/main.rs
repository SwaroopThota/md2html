mod file_helper;
mod md_to_html_converter;

use file_helper::{get_file_name_without_extension, read_file_to_string, write_string_to_file};
use md_to_html_converter::md2html;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Error: Please provide a markdown file path as an argument! \nUsage: md2html_converter <path_to_markdown_file>");

    let md_string = read_file_to_string(&path).expect("Error: Failed to read the markdown file!");

    let file_name = get_file_name_without_extension(&path).unwrap_or(String::from("HTML Document"));
    let html_file_name = format!("{}.html", file_name);
    let html_string = md2html(&md_string, Some(&file_name), None);

    write_string_to_file(&html_file_name, &html_string)
        .expect("Error: Failed to write the HTML file!");

    println!("Successfully converted '{path}' to '{html_file_name}'.");
}
