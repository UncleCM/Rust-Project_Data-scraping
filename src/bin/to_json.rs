#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use reqwest;
use scraper::{Html, Selector};
use serde_json::Value;

fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}

pub fn select_and_save_html_data_to_json(
    file_path: &str,
    output_json: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let html_content = std::fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);
    let mut data = Vec::<Value>::new();

    if selector.starts_with("table") {
        // Try to select by a class-specific table
        if let Ok(selector_result) = Selector::parse(selector) {
            if let Some(table) = document.select(&selector_result).next() {
                for row in table.select(&Selector::parse("tr").unwrap()) {
                    let mut row_data = serde_json::Map::new();

                    for (index, cell) in row.select(&Selector::parse("th, td").unwrap()).enumerate() {
                        let cell_text = cell.text().collect::<String>();
                        let key = format!("col{}", index);
                        row_data.insert(key, Value::String(cell_text));
                    }

                    data.push(Value::Object(row_data));
                }
            }
        } else {
            eprintln!("Error parsing selector for the table.");
        }
    } else if selector.starts_with('.') && !selector.starts_with("table.") {
        // Check if the selector starts with a dot and doesn't start with "table"
        // This indicates a class selector
        if let Ok(selector_result) = Selector::parse(selector) {
            for class_element in document.select(&selector_result) {
                let class_data = class_element.text().collect::<String>();
                data.push(Value::String(class_data));
            }
        } else {
            eprintln!("Error parsing class selector.");
        }
    } else {
        // Direct element tag selector
        let element_selector = Selector::parse(selector);
        if let Ok(element_selector) = element_selector {
            for element in document.select(&element_selector) {
                let element_data = element.text().collect::<String>();
                data.push(Value::String(element_data));
            }
        } else {
            eprintln!("Invalid selector. Please provide a valid class-specific table selector (e.g., 'table.my-table-class'), a class selector (e.g., '.my-class'), or a direct element tag selector (e.g., 'p' or 'h3').");
        }
    }

    let json_data = serde_json::to_string_pretty(&data)?;

    let mut json_file = File::create(output_json)?;
    json_file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";
    let output_filename = "output.json";

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");

    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;

    select_and_save_html_data_to_json(html_filename, output_filename, selector.trim())?;
    println!("Data saved to '{}'.", output_filename);

    Ok(())
}
