#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use reqwest;
use scraper::{Html, Selector};
use csv::Writer;

pub fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}

pub fn select_and_save_html_data_to_csv(
    file_path: &str,
    output_csv: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let html_content = std::fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);

    let mut csv_writer = Writer::from_path(output_csv)?;

    let mut is_header_written = false;

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

                    // Write the row data to the CSV file
                    if !row_data.is_empty() {
                        if !is_header_written {
                            // Write the first row as the header
                            csv_writer.write_record(&row_data)?;
                            is_header_written = true;
                        } else {
                            csv_writer.write_record(&row_data)?;
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
                // Process the class-selected data here and write to the CSV as needed
                let class_data = class_element.text().collect::<String>();
                // Write the class data to the CSV
                csv_writer.write_record(&[class_data])?;
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
                csv_writer.write_record(&[element_data])?;
            }
        } else {
            eprintln!("Invalid selector. Please provide a valid class-specific table selector (e.g., 'table.my-table-class'), a class selector (e.g., '.my-class'), or a direct element tag selector (e.g., 'p' or 'h3').");
        }
    }
    csv_writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";
    let output_filename = "output.csv";

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");
    
    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;
    
    select_and_save_html_data_to_csv(html_filename, output_filename, selector.trim())?;
    println!("Data saved to '{}'.", output_filename);

    Ok(())
}
