use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml_edit::Document;

fn get_toml() -> Option<File> {
    let mut dir = current_dir().unwrap();

    loop {
        let mut cur = dir.clone();

        cur.push("Cargo.toml");
        match File::open(cur) {
            Ok(f) => {
                return Some(f);
            }
            Err(_) => {
                if !dir.pop() {
                    return None;
                }
                continue;
            }
        }
    }
}

fn main() {
    let toml = match get_toml() {
        Some(toml) => toml,
        None => {
            println!("cannot find Cargo.toml");
            return;
        }
    };

    let mut buf_reader = BufReader::new(toml);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let doc = contents.parse::<Document>().expect("invalid doc");
    if let Some(s) = doc["package"]["name"].as_str() {
        let url = format!("{}{}", "https://crates.io/crates/", s);
        open::that(url).expect("url open fails!");
    }
}
