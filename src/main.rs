mod aux;
mod show;
mod check;

use std::env;
use std::process::exit;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 5 {
        println!("[-] Error. Incorrect number of arguments supplied");
        println!("[?] Usage: {} -u <url> -w <wordlist>", argv[0]);
        exit(1);
    }

    check::args(&argv);
    check::url(&argv[2]);
    let entries = check::wordlist(&argv[4]);
    
    show::banner(&argv[2], &entries);

    aux::fuzz(&argv[2], &argv[4]);
}
