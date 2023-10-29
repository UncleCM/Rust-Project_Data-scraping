#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use reqwest;
use scraper::{Html, Selector};
use rmp_serde::Serializer;
use serde::Serialize;

pub fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}

pub fn select_and_save_html_data_to_messagepack<T: Serialize>(
    file_path: &str,
    output_msgpack: &str,
    selector: &str,
    data: T,
) -> Result<(), Box<dyn Error>> {
    let html_content = std::fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);

    if selector.starts_with("table") {
        if let Ok(selector_result) = Selector::parse(selector) {
            if document.select(&selector_result).next().is_some() {
                let mut buf = Vec::new();
                let mut ser = Serializer::new(&mut buf);
                data.serialize(&mut ser)?;

                let mut file = File::create(output_msgpack)?;
                file.write_all(&buf)?;
            } else {
                eprintln!("No table found with the provided selector.");
            }
        } else {
            eprintln!("Error parsing selector for the table.");
        }
    } else if selector.starts_with('.') && !selector.starts_with("table.") {
        if let Ok(selector_result) = Selector::parse(selector) {
            let mut buf = Vec::new();
            let mut ser = Serializer::new(&mut buf);
            data.serialize(&mut ser)?;

            let mut file = File::create(output_msgpack)?;
            file.write_all(&buf)?;
        } else {
            eprintln!("Error parsing class selector.");
        }
    } else {
        eprintln!("Invalid selector. Please provide a valid class-specific table selector (e.g., 'table.my-table-class'), a class selector (e.g., '.my-class'), or a direct element tag selector (e.g., 'p' or 'h3').");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";
    let output_filename = "output.msgpack";

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");
    
    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;

    select_and_save_html_data_to_messagepack(html_filename, output_filename, selector.trim(), selector.trim())?;
    println!("Data saved to '{}'.", output_filename);

    Ok(())
}
