use crate::brainfuck::{evaluate_str};

use std::io::{
    stdin,
    stdout,
    Write,
};

pub fn run_shell(){
    let mut input = String::new();
    let mut tape:Option<Vec<u8>> = Some(Vec::with_capacity(30000));
    println!("brainfuck-rs v3.0.0");
    println!("Brainfuck Interactive Shell");
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        tape = match command {
            "exit" => None,
            _ => evaluate_str(String::from(command), true, tape),
        };
        match tape {
            Some(_) => continue,
            None => break,
        };
    }    
}
