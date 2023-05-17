extern crate interrust;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use interrust::rust_parser;
use interrust::eval;

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();

    println!("InterRust, the Rust Interpreter");
    print!(">>> ");
    io::stdout().flush().unwrap();

    while reader.read_line(&mut line).is_ok() {
        if line.len() == 0 {
            println!("We're done here.");
            break;
        }

        let expression = rust_parser::expr(line.trim());

        match expression {
            Ok(e) => {
                let value = eval(e);
                println!("{}", value);
            },
            Err(e) => {
                println!("{}", e);
            }
        }

        line.clear();
        print!(">>> ");
        io::stdout().flush().unwrap();
    }
}
