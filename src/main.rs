#[allow(dead_code, unused_imports)]
mod bin{
    pub mod to_csv;
    use to_csv::select_and_save_html_data_to_csv;
    pub mod to_json;
    use to_json::select_and_save_html_data_to_json;
    pub mod to_xml;
    use to_xml::select_and_save_html_data_to_xml;
    pub mod to_yaml;
    use to_yaml::select_and_save_html_data_to_yaml;
    pub mod to_toml;
    use to_toml::select_and_save_html_data_to_toml;
    pub mod to_messagepack;
    use to_messagepack::select_and_save_html_data_to_messagepack;
}
use std::error::Error;
use std::io;
use std::fs::File;
use std::io::Write;

fn scrape_and_save_html_to_file(url: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let mut file = File::create(output_file)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the URL of the website you want to scrape:");
    let mut web = String::new();
    io::stdin().read_line(&mut web)?;

    let web_url = web.trim();
    let html_filename = "html_content.html";

    println!("Enter the selector for the data you want to scrape:");
    println!("You can parse table data by providing 'table' or class data by providing a class selector (e.g. '.my-class').");
    let mut selector = String::new();
    io::stdin().read_line(&mut selector)?;

    scrape_and_save_html_to_file(web_url, html_filename)?;
    println!("Scraping and HTML creation completed successfully.");
    println!("Enter the data format you want to save the data in:");
    println!("You can choose from the following options:");
    println!("1. CSV");
    println!("2. JSON");
    println!("3. XML");
    println!("4. YAML");
    println!("5. TOML");
    println!("6. MessagePack");
    let mut data_format = String::new();
    io::stdin().read_line(&mut data_format)?;

    let data_format = data_format.trim();

    let output_filename = match data_format {
        "1" | "CSV" => {
            bin::to_csv::select_and_save_html_data_to_csv(html_filename, "output.csv", selector.trim())?;
            "output.csv"
        }
        "2" | "JSON" => {
            bin::to_json::select_and_save_html_data_to_json(html_filename, "output.json", selector.trim())?;
            "output.json"
        }
        "3" | "XML" => {
            bin::to_xml::select_and_save_html_data_to_xml(html_filename, "output.xml", selector.trim())?;
            "output.xml"
        }
        "4" | "YAML" => {
            bin::to_yaml::select_and_save_html_data_to_yaml(html_filename, "output.yaml", selector.trim())?;
            "output.yaml"
        }
        "5" | "TOML" => {
            bin::to_toml::select_and_save_html_data_to_toml(html_filename, "output.toml", selector.trim())?;
            "output.toml"
        }
        "6" | "MessagePack" => {
            bin::to_messagepack::select_and_save_html_data_to_messagepack(html_filename, "output.mp", selector.trim(),selector.trim())?;
            "output.mp"
        }
        _ => {
            println!("Invalid data format specified.");
            return Ok(());
        }
    };


    println!("Data saved to '{}'.", output_filename);

    Ok(())
}
