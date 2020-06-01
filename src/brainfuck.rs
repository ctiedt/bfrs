use crate::token::{Token, TokenType};

pub fn strip_comments(code: &str) -> String {
    let mut stripped = String::new();
    for character in code.chars() {
        if ['>', '<', '+', '-', ',', '.', '[', ']'].contains(&character) {
            stripped.push(character);
        }
    }
    stripped
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut stack: Vec<usize> = vec![];
    let mut tokens: Vec<Token> = vec![];
    let mut warned = false;
    for (index, character) in code.chars().enumerate() {
        match character {
            '>' => tokens.push(Token::inc_ptr()),
            '<' => tokens.push(Token::dec_ptr()),
            '+' => tokens.push(Token::inc_val()),
            '-' => tokens.push(Token::dec_val()),
            ',' => tokens.push(Token::acc_in()),
            '.' => tokens.push(Token::out()),
            '[' => {
                tokens.push(Token::loop_begin(None));
                stack.push(index);
            }
            ']' => tokens.push(Token::loop_end(stack.pop().unwrap())),
            _ => {
                if !warned {
                    println!("It seems that this code has not been stripped of comments. Loops may not work properly.");
                    warned = true;
                }
            }
        }
    }
    // Counting closing brackets and adding their references is easy, but the other way around is more difficult.
    // The simplest solution (which still leaves tokenization in O(n)) is to do a second pass where we set the correct
    // reference for any opening bracket by finding it through the closing one.
    for index in 0..tokens.len() {
        match tokens[index].referencing {
            Some(referencing) => tokens[referencing] = Token::loop_begin(Some(index)),
            None => {}
        }
    }
    tokens
}

pub fn to_brainfuck(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for token in tokens {
        result.push(match token.token_type {
            TokenType::IncPtr => '>',
            TokenType::DecPtr => '<',
            TokenType::IncVal => '+',
            TokenType::DecVal => '-',
            TokenType::AccIn => ',',
            TokenType::Out => '.',
            TokenType::LoopBegin => '[',
            TokenType::LoopEnd => ']',
        });
    }
    result
}
