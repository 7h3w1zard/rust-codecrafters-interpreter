use crate::token::TokenType;

pub(crate) struct Scanner {
    source: String,
}

impl Scanner {
    pub(crate) fn scan_tokens(source: String) {
        let null = "null";
        
        for c in source.chars() {
            match c {
                '(' => println!("{:?} {c} {null}", TokenType::LEFT_PAREN),
                ')' => println!("{:?} {c} {null}", TokenType::RIGHT_PAREN),
                '{' => println!("{:?} {c} {null}", TokenType::LEFT_BRACE),
                '}' => println!("{:?} {c} {null}", TokenType::RIGHT_BRACE),
                _ => panic!("Unexpected character."),
            }
        }

        println!("EOF  null");
    }
}
