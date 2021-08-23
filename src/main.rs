use std::io::Read;
use std::collections::HashMap;
use rayon::prelude::*;

use std::fs;

use clap::{Arg, App};

const OPCODES:&str= ".,[]<>+-";

fn evaluate(code:String) {
    let code = clean(code);
    let bmap = build_brace_map(code.clone());

    let mut cells:Vec<u8> = Vec::new();
    cells.push(0);
    let mut codeptr:usize = 0;
    let mut cellptr:usize = 0;

    while codeptr < code.len(){
        let command = code[codeptr];
        match command {
            '>' => {
                cellptr += 1;
                if cellptr == cells.len(){
                    cells.push(0);
                }
            },
            '<' => cellptr = if cellptr <= 0 {0} else {cellptr -1},
            '+' => cells[cellptr] = if cells[cellptr] < 255 {cells[cellptr] + 1} else {0},
            '-' => cells[cellptr] = if cells[cellptr] > 0 {cells[cellptr] - 1} else {255},
            '[' => {
                if cells[cellptr] == 0 {
                    codeptr = bmap[&codeptr];
                }
            },
            ']' => {
                if cells[cellptr] != 0 {
                    codeptr = bmap[&codeptr];
                }
            },
            '.' => print!("{}", cells[cellptr] as char),
            ',' => {
                let input:Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(| result | result.ok())
                    .map(|byte| byte as u8);
                cells[cellptr] = input.unwrap();
            },
            _ => {},
        } codeptr += 1;
    }

}

fn clean(code:String) -> Vec<char>{
    code.par_chars()
        .filter(|&c| OPCODES.contains(c))
        .collect()
}

fn build_brace_map(code: Vec<char>) -> HashMap<usize,usize> {
    let mut brace_stack: Vec<usize> = Vec::new();
    let mut bmap: HashMap<usize, usize> = HashMap::new();
    
    for (i, c) in code.iter().enumerate(){
        match c {
            '[' => brace_stack.push(i),
            ']' => {
                let start = brace_stack.pop().unwrap();
                bmap.insert(start, i);
                bmap.insert(i, start);
            }, _ => {}
        }
    } bmap
}

fn execute(filename:&str){
    let contents = fs::read_to_string(filename)
        .expect("cannot read file");
    evaluate(contents);
}


fn main() {
    let matches = App::new("bfrs")
        .version("1.0")
        .author("Ian Kim. <ian@ianmkim.com>")
        .about("A lightning fast brainfuck interpreter written in Rust")
        .arg(Arg::new("file")
             .value_name("FILENAME")
             .about("Brainfuck source file")
             .required(true)
             .index(1))
        .get_matches();

    if let Some(filename) = matches.value_of("file") {
        execute(filename);
    }
}

