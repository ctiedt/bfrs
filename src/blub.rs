use crate::token::{Token, TokenType};

pub fn strip_comments(code: &str) -> String {
    let mut stripped = String::new();
    for character in code.chars() {
        if "Blub.?!".contains(character) {
            stripped.push(character);
        }
    }
    stripped
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut stack: Vec<usize> = vec![];
    let mut tokens: Vec<Token> = vec![];
    let mut index = 0;
    while index * 10 < code.len() {
        let command: &str = &code[index * 10..(index + 1) * 10];
        match (
            command.chars().nth(4).unwrap(),
            command.chars().nth(9).unwrap(),
        ) {
            ('.', '?') => tokens.push(Token::inc_ptr()),
            ('?', '.') => tokens.push(Token::dec_ptr()),
            ('.', '.') => tokens.push(Token::inc_val()),
            ('!', '!') => tokens.push(Token::dec_val()),
            ('!', '.') => tokens.push(Token::out()),
            ('.', '!') => tokens.push(Token::acc_in()),
            ('!', '?') => {
                stack.push(index);
                tokens.push(Token::loop_begin(None));
            }
            ('?', '!') => tokens.push(Token::loop_end(stack.pop().unwrap())),
            _ => {}
        }
        index += 1;
    }
    for index in 0..tokens.len() {
        match tokens[index].referencing {
            Some(referencing) => tokens[referencing] = Token::loop_begin(Some(index)),
            None => {}
        }
    }
    tokens
}

pub fn to_blub(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for token in tokens {
        result.push_str(match token.token_type {
            TokenType::IncPtr => "Blub. Blub? ",
            TokenType::DecPtr => "Blub? Blub. ",
            TokenType::IncVal => "Blub. Blub. ",
            TokenType::DecVal => "Blub! Blub! ",
            TokenType::AccIn => "Blub. Blub! ",
            TokenType::Out => "Blub! Blub. ",
            TokenType::LoopBegin => "Blub! Blub? ",
            TokenType::LoopEnd => "Blub? Blub! ",
        });
    }
    result
}
