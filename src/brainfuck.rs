use crate::Token;

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
            '>' => tokens.push(Token::IncPtr),
            '<' => tokens.push(Token::DecPtr),
            '+' => tokens.push(Token::IncVal),
            '-' => tokens.push(Token::DecVal),
            ',' => tokens.push(Token::AccIn),
            '.' => tokens.push(Token::Out),
            '[' => {
                tokens.push(Token::LoopBegin { referencing: 0 });
                stack.push(index);
            }
            ']' => tokens.push(Token::LoopEnd {
                referencing: stack.pop().unwrap(),
            }),
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
        match tokens[index] {
            Token::LoopEnd { referencing } => {
                tokens[referencing] = Token::LoopBegin { referencing: index }
            }
            _ => {}
        }
    }
    tokens
}

pub fn to_brainfuck(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for token in tokens {
        result.push(match token {
            Token::IncPtr => '>',
            Token::DecPtr => '<',
            Token::IncVal => '+',
            Token::DecVal => '-',
            Token::AccIn => ',',
            Token::Out => '.',
            Token::LoopBegin { referencing: _ } => '[',
            Token::LoopEnd { referencing: _ } => ']',
        });
    }
    result
}
