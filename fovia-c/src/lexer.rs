#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Str(Vec<u8>),
    Number(i64),
    Let,
    Struct,
    Impl,
    Trait,
    Loop,
    Match,
    Brk,
    True,
    False,
    Fn,
    Arrow,
    FatArrow,
    Assign,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    ColonColon,
    Dot,
    Bang,
    Tick,
    Underscore,
    Eof,
}

#[derive(Clone)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_ws_and_comments();
        if self.pos >= self.input.len() {
            return Ok(Token::Eof);
        }

        let ch = self.input[self.pos];
        match ch {
            '<' => {
                if self.peek_char(1) == Some('=') {
                    self.pos += 2;
                    Ok(Token::Le)
                } else {
                    self.pos += 1;
                    Ok(Token::Lt)
                }
            }
            '>' => {
                if self.peek_char(1) == Some('=') {
                    self.pos += 2;
                    Ok(Token::Ge)
                } else {
                    self.pos += 1;
                    Ok(Token::Gt)
                }
            }
            '+' => {
                self.pos += 1;
                Ok(Token::Plus)
            }
            '*' => {
                self.pos += 1;
                Ok(Token::Star)
            }
            '/' => {
                self.pos += 1;
                Ok(Token::Slash)
            }
            '%' => {
                self.pos += 1;
                Ok(Token::Percent)
            }
            '(' => {
                self.pos += 1;
                Ok(Token::LParen)
            }
            ')' => {
                self.pos += 1;
                Ok(Token::RParen)
            }
            '{' => {
                self.pos += 1;
                Ok(Token::LBrace)
            }
            '}' => {
                self.pos += 1;
                Ok(Token::RBrace)
            }
            ',' => {
                self.pos += 1;
                Ok(Token::Comma)
            }
            ':' => {
                if self.peek_char(1) == Some(':') {
                    self.pos += 2;
                    Ok(Token::ColonColon)
                } else {
                    self.pos += 1;
                    Ok(Token::Colon)
                }
            }
            '.' => {
                self.pos += 1;
                Ok(Token::Dot)
            }
            '\'' => {
                self.pos += 1;
                Ok(Token::Tick)
            }
            '!' => {
                if self.peek_char(1) == Some('=') {
                    self.pos += 2;
                    Ok(Token::NotEq)
                } else {
                    self.pos += 1;
                    Ok(Token::Bang)
                }
            }
            '-' => {
                if self.peek_char(1) == Some('>') {
                    self.pos += 2;
                    Ok(Token::Arrow)
                } else {
                    self.pos += 1;
                    Ok(Token::Minus)
                }
            }
            '=' => {
                if self.peek_char(1) == Some('=') {
                    self.pos += 2;
                    Ok(Token::EqEq)
                } else if self.peek_char(1) == Some('>') {
                    self.pos += 2;
                    Ok(Token::FatArrow)
                } else {
                    self.pos += 1;
                    Ok(Token::Assign)
                }
            }
            '"' => {
                let bytes = self.read_string()?;
                Ok(Token::Str(bytes))
            }
            _ if ch.is_ascii_digit() => {
                let number = self.read_number()?;
                Ok(Token::Number(number))
            }
            _ if is_ident_start(ch) => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "fn" => Ok(Token::Fn),
                    "let" => Ok(Token::Let),
                    "struct" => Ok(Token::Struct),
                    "impl" => Ok(Token::Impl),
                    "trait" => Ok(Token::Trait),
                    "loop" => Ok(Token::Loop),
                    "match" => Ok(Token::Match),
                    "brk" => Ok(Token::Brk),
                    "true" => Ok(Token::True),
                    "false" => Ok(Token::False),
                    "_" => Ok(Token::Underscore),
                    _ => Ok(Token::Ident(ident)),
                }
            }
            _ => Err(format!("unexpected character: {ch}")),
        }
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }

            if self.pos + 1 < self.input.len()
                && self.input[self.pos] == '/'
                && self.input[self.pos + 1] == '/'
            {
                self.pos += 2;
                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue;
            }

            break;
        }
    }

    fn read_ident(&mut self) -> String {
        let start = self.pos;
        self.pos += 1;
        while self.pos < self.input.len() && is_ident_continue(self.input[self.pos]) {
            self.pos += 1;
        }
        self.input[start..self.pos].iter().collect()
    }

    fn read_number(&mut self) -> Result<i64, String> {
        let start = self.pos;
        self.pos += 1;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        s.parse::<i64>().map_err(|_| "invalid number".to_string())
    }

    fn read_string(&mut self) -> Result<Vec<u8>, String> {
        self.pos += 1; // skip opening quote
        let mut bytes = Vec::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            self.pos += 1;
            match ch {
                '"' => return Ok(bytes),
                '\\' => {
                    let esc = self.read_escape()?;
                    bytes.extend(esc);
                }
                _ => {
                    let mut buf = [0u8; 4];
                    let s = ch.encode_utf8(&mut buf);
                    bytes.extend(s.as_bytes());
                }
            }
        }
        Err("unterminated string literal".to_string())
    }

    fn read_escape(&mut self) -> Result<Vec<u8>, String> {
        if self.pos >= self.input.len() {
            return Err("unterminated escape".to_string());
        }
        let ch = self.input[self.pos];
        self.pos += 1;
        match ch {
            'n' => Ok(vec![b'\n']),
            'r' => Ok(vec![b'\r']),
            't' => Ok(vec![b'\t']),
            '\\' => Ok(vec![b'\\']),
            '"' => Ok(vec![b'"']),
            'x' => {
                let hi = self.read_hex_digit()?;
                let lo = self.read_hex_digit()?;
                Ok(vec![hi * 16 + lo])
            }
            _ => Err(format!("invalid escape: \\{ch}")),
        }
    }

    fn read_hex_digit(&mut self) -> Result<u8, String> {
        if self.pos >= self.input.len() {
            return Err("incomplete \\x escape".to_string());
        }
        let ch = self.input[self.pos];
        self.pos += 1;
        ch.to_digit(16)
            .map(|v| v as u8)
            .ok_or_else(|| format!("invalid hex digit: {ch}"))
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        self.input.get(self.pos + offset).copied()
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}
