pub fn banner(target: &String, entries: &u32) {
    println!("");
    println!("**********************************************");
    println!("|    rfuzzer - Web fuzzer written in Rust    |");
    println!("**********************************************");
    
    println!("Target: {}", target);
    println!("Requests: {}\n", entries);

    println!("==============================================");
    println!("ID          STATUS          REQUEST           ");
    println!("==============================================");
    println!("");
}
