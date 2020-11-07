use std::env;


pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);

    let command = args[1].clone();
    println!("Args: {:?}", command);

}