use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml_edit::Document;

fn main() {
    let toml = File::open("./Cargo.toml").unwrap();
    let mut buf_reader = BufReader::new(toml);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let doc = contents.parse::<Document>().expect("invalid doc");
    match doc["package"]["name"].as_str() {
        Some(s) => {
            let url = format!("{}{}", "https://crates.io/crates/", s);
            open::that(url).expect("url open fails!");
        }
        None => {}
    }
}
