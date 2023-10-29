#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
struct CustomData {
    #[serde(rename = "table_data")]
    table_data: Option<TableRow>,
    #[serde(rename = "class_data")]
    class_data: Option<String>,
    #[serde(rename = "element_data")]
    element_data: Option<String>,
}

#[derive(Serialize)]
struct TableRow {
    #[serde(rename = "row")]
    rows: Vec<(String, String)>,
}

fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}

pub fn select_and_save_html_data_to_xml(
    file_path: &str,
    output_xml: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let html_content = std::fs::read_to_string(file_path)?;

    let document = Html::parse_document(&html_content);

    let mut custom_data = CustomData {
        table_data: None,
        class_data: None,
        element_data: None,
    };

    if selector.starts_with("table") {
        if let Ok(selector_result) = Selector::parse(selector) {
            if let Some(table) = document.select(&selector_result).next() {
                let mut rows = Vec::<(String, String)>::new();

                for row in table.select(&Selector::parse("tr").unwrap()) {
                    let mut row_data = Vec::<(String, String)>::new();

                    for (index, cell) in row.select(&Selector::parse("th, td").unwrap()).enumerate() {
                        let cell_text = cell.text().collect::<String>();
                        let key = format!("col{}", index);
                        row_data.push((key, cell_text));
                    }

                    println!("Debug: row_data: {:?}", row_data); // Add this debug print

                    if let Ok(row_data_xml) = serde_xml_rs::to_string(&row_data) {
                        rows.push(("row".to_string(), row_data_xml));
                    } else {
                        eprintln!("Error serializing row_data to XML");
                    }
                }

                custom_data.table_data = Some(TableRow { rows });
            }
        } else {
            eprintln!("Error parsing selector for the table.");
        }
    } else if selector.starts_with('.') && !selector.starts_with("table.") {
        if let Ok(selector_result) = Selector::parse(selector) {
            for class_element in document.select(&selector_result) {
                let class_data = class_element.text().collect::<String>();
                custom_data.class_data = Some(class_data);
            }
        } else {
            eprintln!("Error parsing class selector.");
        }
    } else {
        let element_selector = Selector::parse(selector);
        if let Ok(element_selector) = element_selector {
            for element in document.select(&element_selector) {
                let element_data = element.text().collect::<String>();
                custom_data.element_data = Some(element_data);
            }
        } else {
            eprintln!("Invalid selector. Please provide a valid class-specific table selector (e.g., 'table.my-table-class'), a class selector (e.g., '.my-class'), or a direct element tag selector (e.g., 'p' or 'h3').");
        }
    }

    let xml = serde_xml_rs::to_string(&custom_data)?;

    let mut xml_file = File::create(output_xml)?;
    xml_file.write_all(xml.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";
    let output_filename = "output.xml";

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");

    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;

    // println!("Debug: Selector is '{}'", selector.trim()); // Add this debug print

    select_and_save_html_data_to_xml(html_filename, output_filename, selector.trim())?;
    //println!("Data saved to '{}'.", output_filename);

    Ok(())
}
