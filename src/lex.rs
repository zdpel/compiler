use crate::errors::TokenError;

#[derive(Debug)]
pub enum Token {
    ID(String, usize),
    Print(usize),
    Num(String, usize),
    Semicolon(usize),
    Assign(usize),
    RightParen(usize),
    LeftParen(usize),
    RightCurl(usize),
    LeftCurl(usize),
    Comma(usize),
    Add(usize),
    Subtract(usize),
    Multiply(usize),
    Divide(usize),
    EndOfFile(usize),
    For(usize),
    LessThan(usize),
    GreaterThan(usize),
}

impl Token {
    pub fn name(&self) -> &str {
        match self {
            Token::ID(_, _) => "ID",
            Token::Print(_) => "Print",
            Token::Num(_, _) => "Num",
            Token::Semicolon(_) => "Semicolon",
            Token::Assign(_) => "Assign",
            Token::RightParen(_) => "RightParen",
            Token::LeftParen(_) => "LeftParen",
            Token::RightCurl(_) => "RightCurl",
            Token::LeftCurl(_) => "LeftCurl",
            Token::Comma(_) => "Comma",
            Token::Add(_) => "Add",
            Token::Subtract(_) => "Subtract",
            Token::Multiply(_) => "Multiply",
            Token::Divide(_) => "Divide",
            Token::EndOfFile(_) => "EndOfFile",
            Token::For(_) => "For",
            Token::LessThan(_) => "LessThan",
            Token::GreaterThan(_) => "GreaterThan",
        }
    }
    pub fn line(&self) -> usize {
        match self {
            Token::ID(_, line) => *line,
            Token::Print(line) => *line,
            Token::Num(_, line) => *line,
            Token::Semicolon(line) => *line,
            Token::Assign(line) => *line,
            Token::RightParen(line) => *line,
            Token::LeftParen(line) => *line,
            Token::RightCurl(line) => *line,
            Token::LeftCurl(line) => *line,
            Token::Comma(line) => *line,
            Token::Add(line) => *line,
            Token::Subtract(line) => *line,
            Token::Multiply(line) => *line,
            Token::Divide(line) => *line,
            Token::EndOfFile(line) => *line,
            Token::For(line) => *line,
            Token::LessThan(line) => *line,
            Token::GreaterThan(line) => *line,
        }
    }
    pub fn val(&self) -> String {
        match self {
            Token::ID(val, _) => val.clone(),
            Token::Num(val, _) => val.clone(),
            _ => String::new(),
        }
    }
}

pub fn is_delimiter(chr: char) -> bool {
    if chr == ' '
        || chr == '('
        || chr == ')'
        || chr == '}'
        || chr == '{'
        || chr == '/'
        || chr == '+'
        || chr == '-'
        || chr == '$'
        || chr == '*'
        || chr == ';'
        || chr == ','
        || chr == '<'
        || chr == '>'
    {
        return true;
    } else {
        return false;
    }
}

pub fn make_delim_token(chr: char, line: usize) -> (Token, bool) {
    match chr {
        '(' => (Token::LeftParen(line), true),
        ')' => (Token::RightParen(line), true),
        '{' => (Token::LeftCurl(line), true),
        '}' => (Token::RightCurl(line), true),
        '/' => (Token::Divide(line), true),
        '+' => (Token::Add(line), true),
        '-' => (Token::Subtract(line), true),
        '*' => (Token::Multiply(line), true),
        ';' => (Token::Semicolon(line), true),
        ',' => (Token::Comma(line), true),
        '<' => (Token::LessThan(line), true),
        '>' => (Token::GreaterThan(line), true),
        _ => {
            return (Token::EndOfFile(line), false);
        }
    }
}

pub fn tokenize(str: String) -> Result<Vec<Token>, TokenError> {
    let mut tok_list: Vec<Token> = Vec::new();
    let mut state: u8 = 0;
    let mut iter1 = str.chars();
    let mut stop = false;

    let mut tok = String::new();

    let mut chr = iter1.next().unwrap_or('$');

    let mut curr_line = 1;

    while stop == false {
        let asc = chr.to_ascii_lowercase() as u16;
        match state {
            0 => {
                if asc >= 97 && asc <= 122 {
                    state = 1;
                    tok.push(chr);
                } else if asc >= 48 && asc <= 57 {
                    state = 3;
                    tok.push(chr);
                } else if is_delimiter(chr) {
                    let (delim_token, tokenable) = make_delim_token(chr, curr_line);
                    if tokenable {
                        tok_list.push(delim_token);
                    }
                } else if asc == 58 {
                    state = 5;
                    tok.push(chr);
                } else if asc == 13 || asc == 10 {
                    if asc == 10 {
                        curr_line = curr_line + 1;
                    }
                } else {
                    return Err(TokenError::InvalidCharacter(chr));
                }
            }
            1 => {
                if (asc >= 97 && asc <= 122) || (asc >= 48 && asc <= 57) || (asc == 95) {
                    tok.push(chr);
                } else if is_delimiter(chr) {
                    match tok.as_str() {
                        "print" => tok_list.push(Token::Print(curr_line)),
                        "for" => tok_list.push(Token::For(curr_line)),
                        _ => tok_list.push(Token::ID(tok.clone(), curr_line)),
                    }
                    tok.clear();

                    let (delim_token, tokenable) = make_delim_token(chr, curr_line);
                    if tokenable {
                        tok_list.push(delim_token);
                    }
                    state = 0;
                } else if asc == 58 {
                    match tok.as_str() {
                        "print" => tok_list.push(Token::Print(curr_line)),
                        "for" => tok_list.push(Token::For(curr_line)),
                        _ => tok_list.push(Token::ID(tok.clone(), curr_line)),
                    }
                    tok.clear();

                    state = 5;
                    tok.push(chr);
                } else if asc == 13 || asc == 10 {
                } else {
                    return Err(TokenError::InvalidCharacter(chr));
                }
            }
            3 => {
                if asc >= 48 && asc <= 57 {
                    tok.push(chr);
                } else if asc == 46 {
                    state = 4;
                    tok.push(chr);
                } else if is_delimiter(chr) {
                    tok_list.push(Token::Num(tok.clone(), curr_line));
                    tok.clear();

                    let (delim_token, tokenable) = make_delim_token(chr, curr_line);
                    if tokenable {
                        tok_list.push(delim_token);
                    }

                    state = 0;
                } else if asc == 13 || asc == 10 {
                } else {
                    return Err(TokenError::InvalidCharacter(chr));
                }
            }
            4 => {
                if asc >= 48 && asc <= 57 {
                    tok.push(chr);
                } else if is_delimiter(chr) {
                    tok_list.push(Token::Num(tok.clone(), curr_line));
                    tok.clear();

                    let (delim_token, tokenable) = make_delim_token(chr, curr_line);
                    if tokenable {
                        tok_list.push(delim_token);
                    }

                    state = 0;
                } else if asc == 13 || asc == 10 {
                } else {
                    return Err(TokenError::InvalidCharacter(chr));
                }
            }
            5 => {
                if asc == 61 {
                    tok.clear();
                    tok_list.push(Token::Assign(curr_line));

                    state = 0;
                } else {
                    return Err(TokenError::InvalidCharacter(chr));
                }
            }
            _ => println!("No matching state"),
        }
        if chr == '$' {
            stop = true;
        }
        chr = iter1.next().unwrap_or('$');
    }
    tok_list.push(Token::EndOfFile(curr_line));
    Ok(tok_list)
}
