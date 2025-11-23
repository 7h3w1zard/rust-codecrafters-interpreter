use std::{fmt, str::FromStr};

#[derive(Debug)]
pub(crate) enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ReservedKeywords {
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

pub(crate) trait Reserved {
    fn is_reserved(s: &str) -> Option<ReservedKeywords>;
}

 impl Reserved for ReservedKeywords {
    fn is_reserved(s: &str) -> Option<ReservedKeywords> {
        match s {
            "and" => Some(ReservedKeywords::AND),
            "class" => Some(ReservedKeywords::CLASS),
            "else" => Some(ReservedKeywords::ELSE),
            "false" => Some(ReservedKeywords::FALSE),
            "for" => Some(ReservedKeywords::FOR),
            "fun" => Some(ReservedKeywords::FUN),
            "if" => Some(ReservedKeywords::IF),
            "nil" => Some(ReservedKeywords::NIL),
            "or" => Some(ReservedKeywords::OR),
            "print" => Some(ReservedKeywords::PRINT),
            "return" => Some(ReservedKeywords::RETURN),
            "super" => Some(ReservedKeywords::SUPER),
            "this" => Some(ReservedKeywords::THIS),
            "true" => Some(ReservedKeywords::TRUE),
            "var" => Some(ReservedKeywords::VAR),
            "while" => Some(ReservedKeywords::WHILE),
            _ => None
        }
    }
}
