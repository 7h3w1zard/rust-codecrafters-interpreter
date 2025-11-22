use std::process::exit;

use crate::token::TokenType;

pub(crate) struct Scanner {
    // source: String,
    // line: u64,
}

impl Scanner {
    pub(crate) fn scan_tokens(source: String) {
        let e = '=';
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
                    if tokens.peek() == Some(&c) {
                        println!("{:?} {c}{c} {null}", TokenType::EQUAL_EQUAL);
                        tokens.next();
                    } else {
                        println!("{:?} {c} {null}", TokenType::EQUAL)
                    }
                }
                '!' => {
                    if tokens.peek() == Some(&e) {
                        println!("{:?} {c}{e} {null}", TokenType::BANG_EQUAL);
                        tokens.next();
                    } else {
                        println!("{:?} {c} {null}", TokenType::BANG)
                    }
                }
                '<' => {
                    if tokens.peek() == Some(&e) {
                        println!("{:?} {c}{e} {null}", TokenType::LESS_EQUAL);
                        tokens.next();
                    } else {
                        println!("{:?} {c} {null}", TokenType::LESS)
                    }
                }
                '>' => {
                    if tokens.peek() == Some(&e) {
                        println!("{:?} {c}{e} {null}", TokenType::GREATER_EQUAL);
                        tokens.next();
                    } else {
                        println!("{:?} {c} {null}", TokenType::GREATER)
                    }
                }
                '/' => {
                    if tokens.peek() == Some(&c) {
                        while let Some(t) = tokens.next() {
                            match t {
                                '\n' => {
                                    line += 1;
                                    break;
                                }
                                _ => {}
                            }
                            if tokens.peek() == None {
                                break;
                            }
                        }
                    } else {
                        println!("{:?} {c} {null}", TokenType::SLASH)
                    }
                }
                '"' => {
                    let mut unterminated = true;
                    let mut string = String::new();
                    while let Some(t) = tokens.next() {
                        if t == c {
                            unterminated = false;
                            break;
                        }
                        string.push(t);
                    }
                    if unterminated {
                        eprintln!("[line {line}] Error: Unterminated string.");
                        had_error = true;
                    } else {
                        println!("{:?} \"{string}\" {string}", TokenType::STRING)
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => line += 1,
                _ => {
                    eprintln!("[line {line}] Error: Unexpected character: {c}");
                    had_error = true;
                }
            }
        }

        println!("EOF  null");

        if had_error {
            exit(65)
        }
    }
}
