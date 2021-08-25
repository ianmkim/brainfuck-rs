use std::io::Read;
use hashbrown::HashMap;
use rayon::prelude::*;
use std::fs;

const OPCODES:&str= ".,[]<>+-";

pub fn evaluate_vec(code:Vec<char>, out:bool){
    let bmap = build_brace_map(code.clone());

    let mut cells:Vec<u8> = Vec::with_capacity(30000);
    let mut codeptr:usize = 0;
    let mut cellptr:usize = 0;

    cells.push(0);
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
            '+' => cells[cellptr] = cells[cellptr]+1 % 255,
            '-' => cells[cellptr] = cells[cellptr]-1 % 255,
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
            '.' => if out {print!("{}", cells[cellptr] as char)},
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

pub fn evaluate_str(code:String, out:bool) {
    let code = clean(code);
    evaluate_vec(code, out);    
}

pub fn clean(code:String) -> Vec<char>{
    code.par_chars()
        .filter(|&c| OPCODES.contains(c))
        .collect()
}

fn build_brace_map(code: Vec<char>) -> HashMap<usize,usize> {
    let mut brace_stack: Vec<usize> = Vec::new();
    let mut bmap: HashMap<usize, usize> = HashMap::new();
    
    for (i, c) in code.iter()
        .enumerate()
        .filter(|(i, &c)| c == '[' || c == ']')
        .map(|(i, &c)| (i, c)){

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

pub fn execute(filename:&str, out:bool){
    let contents = fs::read_to_string(filename)
        .expect("cannot read file");
    let cleaned = clean(contents); 
    evaluate_vec(cleaned, out);
}

pub fn execute_directly_to_vec(filename: &str, out:bool){
    let mut file = fs::File::open(filename).unwrap();
    let mut s: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut s).unwrap();
    let chars : Vec<char> = s.iter()
        .map(|b| *b as char)
        .filter(|&c| OPCODES.contains(c)) 
        .collect::<Vec<_>>();
    evaluate_vec(chars, out);
}
