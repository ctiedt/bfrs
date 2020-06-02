use crate::Token;

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
            ('.', '?') => tokens.push(Token::IncPtr),
            ('?', '.') => tokens.push(Token::DecPtr),
            ('.', '.') => tokens.push(Token::IncVal),
            ('!', '!') => tokens.push(Token::DecVal),
            ('!', '.') => tokens.push(Token::Out),
            ('.', '!') => tokens.push(Token::AccIn),
            ('!', '?') => {
                stack.push(index);
                tokens.push(Token::LoopBegin { referencing: 0 });
            }
            ('?', '!') => tokens.push(Token::LoopEnd {
                referencing: stack.pop().unwrap(),
            }),
            _ => {}
        }
        index += 1;
    }
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

pub fn to_blub(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for token in tokens {
        result.push_str(match token {
            Token::IncPtr => "Blub. Blub? ",
            Token::DecPtr => "Blub? Blub. ",
            Token::IncVal => "Blub. Blub. ",
            Token::DecVal => "Blub! Blub! ",
            Token::AccIn => "Blub. Blub! ",
            Token::Out => "Blub! Blub. ",
            Token::LoopBegin { referencing: _ } => "Blub! Blub? ",
            Token::LoopEnd { referencing: _ } => "Blub? Blub! ",
        });
    }
    result
}
