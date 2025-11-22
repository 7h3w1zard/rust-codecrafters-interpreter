use std::process::exit;

use crate::token::TokenType;

pub(crate) struct Scanner {
    // source: String,
    // line: u64,
}

impl Scanner {
    pub(crate) fn scan_tokens(source: String) {
        let null = "null";
        let mut line = 1;
        let mut had_error = false;

        let mut tokens = source.chars().peekable();

        while let Some(c) = tokens.next() {
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
                '=' => {
                    if tokens.peek() == Some(&'=') {
                        println!("{:?} {c}{c} {null}", TokenType::EQUAL_EQUAL);
                        tokens.next();
                    } else {
                        println!("{:?} {c} {null}", TokenType::EQUAL)
                    }
                },
                '\n' => line += 1,
                _ => {
                    eprintln!("[line {line}] Error: Unexpected character: {c}");
                    had_error = true;
                },
            }
        }

        println!("EOF  null");

        if had_error {
            exit(65)
        }
    }
}
