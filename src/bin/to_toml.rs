#![allow(unused_imports)]
use std::error::Error;
use std::fs::File;
use std::io::Write;
use scraper::{Html, Selector};
use toml::{Value, to_string_pretty};
use toml::Table;
use std::io;

pub fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}

pub fn select_and_save_html_data_to_toml(
    file_path: &str,
    output_toml: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let html_content = std::fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);

    // Create a TOML Value that will hold the data
    let mut toml_data = Value::Table(Table::new());

    if selector.starts_with("table") {
        // Your existing code for table selection

        // Instead of writing to a CSV, add the data to the TOML Value
        // For example, you can store the data as an array of tables in TOML
        let mut table_data = Vec::new();
        for row in document.select(&Selector::parse("tr").unwrap()) {
            let mut row_data = Vec::new();
            for cell in row.select(&Selector::parse("th, td").unwrap()) {
                let cell_text = cell.text().collect::<String>();
                row_data.push(Value::String(cell_text));
            }
            table_data.push(Value::Array(row_data));
        }
        toml_data.as_table_mut().unwrap().insert("table_data".to_string(), Value::Array(table_data));
    } else if selector.starts_with('.') && !selector.starts_with("table.") {
        // Your existing code for class selector

        // Instead of writing to a CSV, add the data to the TOML Value
        let mut class_data = Vec::new();
        for class_element in document.select(&Selector::parse(selector).unwrap()) {
            let class_text = class_element.text().collect::<String>();
            class_data.push(Value::String(class_text));
        }
        toml_data.as_table_mut().unwrap().insert("class_data".to_string(), Value::Array(class_data));
    } else {
        // Your existing code for direct element tag selector

        // Instead of writing to a CSV, add the data to the TOML Value
        let mut element_data = Vec::new();
        for element in document.select(&Selector::parse(selector).unwrap()) {
            let element_text = element.text().collect::<String>();
            element_data.push(Value::String(element_text));
        }
        toml_data.as_table_mut().unwrap().insert("element_data".to_string(), Value::Array(element_data));
    }

    // Serialize the TOML Value to a TOML string
    let toml_string = to_string_pretty(&toml_data)?;

    // Write the TOML string to the output file
    std::fs::write(output_toml, toml_string)?;

    Ok(())
}
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";
    let output_filename = "output.toml";

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");

    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;

    select_and_save_html_data_to_toml(html_filename, output_filename, selector.trim())?;
    println!("Data saved to '{}'.", output_filename);

    Ok(())
}