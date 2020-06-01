mod blub;
mod brainfuck;
mod token;
use token::{Token, TokenType};

fn accept_input() -> u8 {
    use std::io::stdin;
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    match buf.parse::<u8>() {
        Ok(v) => v,
        Err(_) => 0,
    }
}

fn run(code: Vec<Token>) {
    let mut data: Vec<u8> = vec![0];
    let mut dp: usize = 0;
    let mut ip = 0;
    while ip < code.len() {
        let instruction = code.get(ip).unwrap();
        match instruction.token_type {
            TokenType::IncPtr => {
                if dp >= data.len() - 1 {
                    data.extend(&[0]);
                }
                dp += 1;
            }
            TokenType::DecPtr => dp -= 1,
            TokenType::IncVal => data[dp] += 1,
            TokenType::DecVal => data[dp] -= 1,
            TokenType::AccIn => data[dp] = accept_input(),
            TokenType::Out => print!("{}", char::from(data[dp])),
            TokenType::LoopBegin => {
                if data[dp] == 0 {
                    ip = instruction.referencing.unwrap();
                }
            }
            TokenType::LoopEnd => {
                if data[dp] != 0 {
                    ip = instruction.referencing.unwrap();
                }
            }
        }
        ip += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Use 'bfrs <infile.(bf|blub)> to run a program'");
        println!("Add '--to-(brainfuck|blub)' to convert the input to the language you want");
        return;
    }
    let filename: &String = args.get(1).unwrap();
    let file = String::from_utf8(std::fs::read(filename).unwrap()).unwrap();
    let stripped: String;
    let tokens: Vec<Token>;
    if filename.ends_with(".bf") {
        stripped = brainfuck::strip_comments(&file);
        tokens = brainfuck::tokenize(&stripped);
    } else if filename.ends_with(".blub") {
        stripped = blub::strip_comments(&file);
        tokens = blub::tokenize(&stripped);
    } else {
        panic!("Unrecognized format: {}", filename);
    };
    match args.get(2) {
        Some(arg) => {
            if arg == &String::from("--to-blub") {
                println!("{}", blub::to_blub(tokens));
            } else if arg == &String::from("--to-brainfuck") {
                println!("{}", brainfuck::to_brainfuck(tokens));
            }
        }
        None => run(tokens),
    }
}
