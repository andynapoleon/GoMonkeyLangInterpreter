use std::env;
use std::io;
use std::io::BufReader;
use std::process;

use rust_monkey::repl::*;

fn main() {
    let user = match env::var("USER") {
        Ok(username) => username,
        Err(_) => {
            eprintln!("Failed to get current user");
            process::exit(1);
        }
    };
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");
    start(BufReader::new(io::stdin()), io::stdout());
}
