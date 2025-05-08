use std::fmt;

use crate::lex::Token;

pub enum TokenError {
    InvalidCharacter(char),
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::InvalidCharacter(c) => write!(f, "Invalid input found: {}", c),
        }
    }
}

pub enum ParseError<'a> {
    UnexpectedToken(&'a Token, &'a str, &'a usize),
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedToken(tok, err_mess, pos) => {
                write!(
                    f,
                    "Unexpected token at line {}, pos in token stream: {}: {} {}",
                    tok.line(),
                    pos,
                    tok.name(),
                    err_mess
                )
            }
        }
    }
}

pub fn throw_err<'a>(curr_tok: &'a Token, pos: &'a usize) -> ParseError<'a> {
    let mut err_message = "";
    match curr_tok {
        Token::EndOfFile(_) => {
            err_message = ": Potential problem: statement has unneeded semicolon at end of program or for-loop";
        }
        Token::RightCurl(_) => {
            err_message = ": Potential problem: empty For-block not allowed or unneeded semicolon";
        }
        Token::ID(val, _) => {
            err_message = val;
        }
        Token::Num(val, _) => {
            err_message = val;
        }
        _ => {}
    }
    return ParseError::UnexpectedToken(curr_tok, err_message, pos);
}
