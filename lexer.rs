#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num,
    Add,
    Sub,
    Div,
    Mul,
    Dot,
    Opt,
    Cpt,
    Scln,
    Equ, 
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

pub fn lex(file_buffer: &str, pos: &mut usize) -> Option<Token> {
    let chars: Vec<char> = file_buffer.chars().collect();

    while *pos < chars.len() {
        if chars[*pos].is_whitespace() {
            *pos += 1;
            continue;
        }

        if chars[*pos].is_ascii_digit() {
            let mut val = String::new();
            let mut float = false;
            while *pos < chars.len() &&
                  (chars[*pos].is_ascii_digit() || chars[*pos] == '.') {
                if chars[*pos] == '.' {
                    if float {
                        println!("Error: Invalid number (multiple dots)");
                        return None;
                    }
                    float = true;
                }
                val.push(chars[*pos]);
                *pos += 1;
            }
            return Some(Token { ttype: TokenType::Num, value: val });
        }

        // Match single-character tokens
        let tok = match chars[*pos] {
            '+' => Some(Token { ttype: TokenType::Add, value: "+".to_string() }),
            '-' => Some(Token { ttype: TokenType::Sub, value: "-".to_string() }),
            '*' => Some(Token { ttype: TokenType::Mul, value: "*".to_string() }),
            '/' => Some(Token { ttype: TokenType::Div, value: "/".to_string() }),
            '=' => Some(Token { ttype: TokenType::Equ, value: "=".to_string() }),
            '(' => Some(Token { ttype: TokenType::Opt, value: "(".to_string() }),
            ')' => Some(Token { ttype: TokenType::Cpt, value: ")".to_string() }),
            ';' => Some(Token { ttype: TokenType::Scln, value: ";".to_string() }),
            _ => None,
        };

        *pos += 1;

        return tok;
    }

    None
}
