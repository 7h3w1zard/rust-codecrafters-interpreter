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
                ',' => println!("{:?} {c} {null}", TokenType::COMMA),
                '.' => println!("{:?} {c} {null}", TokenType::DOT),
                '-' => println!("{:?} {c} {null}", TokenType::MINUS),
                '+' => println!("{:?} {c} {null}", TokenType::PLUS),
                ';' => println!("{:?} {c} {null}", TokenType::SEMICOLON),
                '*' => println!("{:?} {c} {null}", TokenType::STAR),
                _ => panic!("Unexpected character."),
            }
        }

        println!("EOF  null");
    }
}
