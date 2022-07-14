use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use std::io::prelude::*;

pub fn args(argv: &Vec<String>) {
    if argv[1] != "-u" {
        println!("[-] Unknown argument '{}'", argv[1]);
        println!("[?] Did you mean '-u'?");
        exit(2);
    }

     if argv[3] != "-w" {
        println!("[-] Unknown argument '{}'", argv[3]);
        println!("[?] Did you mean '-w'?");
        exit(2);
    }
}

pub fn url(urlpath: &String) {
    if urlpath.contains("http://") == false && urlpath.contains("https://") == false {
        println!("[-] Missing http / https protocol");
        println!("[?] Example: https://mysite.com/FUZZ");
        exit(3);
    }

    if urlpath.contains("FUZZ") == false {
        println!("[-] Did not found keyword FUZZ");
        println!("[?] Example: http://mysite.com/FUZZ");
        exit(3);
    }
}

pub fn wordlist(pathname: &String) -> u32 {
    let file = File::open(pathname)
        .expect("Failed to open file");

    let reader = BufReader::new(file);

    let mut entries = 0;
    for _ in reader.lines() {
        entries += 1;
    }

    entries
}
