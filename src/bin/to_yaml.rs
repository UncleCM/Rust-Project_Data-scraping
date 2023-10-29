#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use reqwest;
use scraper::{Html, Selector};
use serde_yaml;
use serde::Serialize;

#[derive(Serialize)]
struct Data {
    items: Vec<String>,
}

pub fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}

pub fn select_and_save_html_data_to_yaml(
    file_path: &str,
    output_yaml: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let html_content = std::fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);

    let mut data = Data { items: Vec::new() };

    if selector.starts_with("table") {
        // Try to select by a class-specific table
        if let Ok(selector_result) = Selector::parse(selector) {
            if let Some(table) = document.select(&selector_result).next() {
                for row in table.select(&Selector::parse("tr").unwrap()) {
                    let mut row_data = Vec::new();

                    for cell in row.select(&Selector::parse("th, td").unwrap()) {
                        let cell_text = cell.text().collect::<String>();
                        row_data.push(cell_text);
                    }

                    // Write the row data to the Data struct
                    if !row_data.is_empty() {
                        if data.items.is_empty() {
                            // Write the first row as the header
                            data.items.push(row_data.join(", "));
                        } else {
                            data.items.push(row_data.join(", "));
                        }
                    }
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
                println!("Element content: {:?}", class_element.text().collect::<String>());
                // Process the class-selected data here and write to the Data struct as needed
                let class_data = class_element.text().collect::<String>();
                data.items.push(class_data);
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
                data.items.push(element_data);
            }
        } else {
            eprintln!("Invalid selector. Please provide a valid class-specific table selector (e.g., 'table.my-table-class'), a class selector (e.g., '.my-class'), or a direct element tag selector (e.g., 'p' or 'h3').");
        }
    }

    // Serialize the Data struct to YAML and write it to the output YAML file
    let yaml_data = serde_yaml::to_string(&data)?;
    let mut file = File::create(output_yaml)?;
    file.write_all(yaml_data.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";
    let output_file = "output.yaml";

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");
    
    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;
    
    select_and_save_html_data_to_yaml(html_filename, output_file, selector.trim())?;
    println!("Data saved to '{}'.", output_file);

    Ok(())
}
