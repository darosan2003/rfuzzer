use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

pub fn fuzz(url: &String, filename: &String) {
    let file = File::open(filename)
        .unwrap();

    let re = Regex::new(r"\d+")
        .unwrap();

    let reader = BufReader::new(file);
    let mut i = 1;

    for entry in reader.lines() {
        let entry = entry.unwrap();
        let req = url.replace("FUZZ", entry.as_str());
    
        match reqwest::get(req.as_str()) {
            Ok(response) => {
                let status = format!("{}", response.status());
                
                if status.contains("404") == true {
                    i += 1;
                    continue;
                }

                let status = re.find(status.as_str())
                    .unwrap()
                    .as_str();

                println!("{:08}    {:15} /{}/", i, status, entry.as_str());
                i += 1;
            },
            Err(_) => {
                println!("[-] Failed to send request");
                i += 1;
                continue;
            }
        }
    }
}
