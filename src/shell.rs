use crate::brainfuck::{evaluate_str};

use std::io::{
    stdin,
    stdout,
    Write,
};

pub fn run_shell(){
    let mut input = String::new();
    println!("brainfuck-rs v2.0.0");
    println!("Brainfuck Interactive Shell");
    loop {
        print!("> ");
        stdout().flush();
        stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        match command {
            "exit" => break,
            _ => evaluate_str(String::from(command), true),
        }
    }    
}
