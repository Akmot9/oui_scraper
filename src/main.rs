use reqwest::blocking::get;
use std::error::Error;
use csv::Writer;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn fetch_oui_data(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?.text()?;
    println!("Fetched response from {}", url);
    Ok(response)
}

fn parse_oui_data(data: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let lines: Vec<&str> = data.lines().collect();

    for i in 0..lines.len() {
        if lines[i].contains("(hex)") {
            let hex_part = lines[i][..lines[i].find("(hex)").unwrap()].trim().to_string();
            let company_part = lines[i][lines[i].find("(hex)").unwrap() + 5..].trim().to_string();
            results.push((hex_part, company_part));
        }
    }

    results
}

fn write_to_csv(data: Vec<(String, String)>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;
    wtr.write_record(&["Hex", "Company"])?;

    for (hex, company) in data {
        wtr.write_record(&[hex, company])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://standards-oui.ieee.org/oui/oui.txt";
    let data = fetch_oui_data(url)?;
    let oui_data = parse_oui_data(&data);
    write_to_csv(oui_data, "oui_data.csv")?;
    println!("Data has been written to oui_data.csv");
    Ok(())
}
