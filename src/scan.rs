use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Multiply,
    Divide,
    Power,
    Modulo,
    Equal,
    Factorial,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Variable,
    SpecialVariable,
    BuiltinFunction,
    Integer(u64),
    Float(f64),
    Comma,
    End,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
}

impl Token<'_> {
    pub fn new(kind: TokenKind, lexeme: &str) -> Token<'_> {
        Token { kind, lexeme }
    }
}

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: VecDeque<Token<'a>>,
    start: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source.as_bytes(),
            tokens: VecDeque::new(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan_all(mut self) -> Result<Vec<Token<'a>>, &'static str> {
        while self.next()?.kind != TokenKind::End {}
        Ok(self.tokens.drain(0..).collect())
    }

    pub fn peek_token<'b>(&'b mut self, idx: usize) -> Result<&'b Token<'a>, &'static str> {
        if self.tokens.is_empty() {
            self.next()?;
        }

        while self.tokens.len() <= idx {
            self.next()?;
        }

        Ok(&self.tokens[idx])
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, &'static str> {
        if self.tokens.is_empty() {
            self.next()?;
        }

        Ok(self.tokens.pop_front().unwrap())
    }

    fn next<'b>(&'b mut self) -> Result<&'b Token<'a>, &'static str> {
        self.slurp_whitespace();
        if self.is_at_end() {
            self.add_token(TokenKind::End)?;
            return Ok(&self.tokens[self.tokens.len() - 1]);
        }

        self.start = self.current;
        let tk = match self.advance_char() {
            b'(' => TokenKind::LeftParen,
            b')' => TokenKind::RightParen,
            b'-' => TokenKind::Minus,
            b'+' => TokenKind::Plus,
            b'*' => TokenKind::Multiply,
            b'/' => TokenKind::Divide,
            b'^' => TokenKind::Power,
            b'%' => TokenKind::Modulo,
            b'=' => TokenKind::Equal,
            b',' => TokenKind::Comma,
            b'<' => {
                if self.peek_char() == b'=' {
                    self.advance_char();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.advance_char();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.advance_char();
                    TokenKind::NotEqual
                } else {
                    TokenKind::Factorial
                }
            }
            c => {
                if c.is_ascii_digit() {
                    self.scan_number()?
                } else if c.is_ascii_whitespace() {
                    panic!("shouldn't be whitespace here :(");
                } else {
                    self.scan_name()?
                }
            }
        };

        self.add_token(tk)?;
        Ok(&self.tokens[self.tokens.len() - 1])
    }

    fn scan_name(&mut self) -> Result<TokenKind, &'static str> {
        while !is_non_identifier(self.peek_char()) {
            self.advance_char();
        }

        if let Some(tk) = to_keyword(self.lexeme()?) {
            Ok(tk)
        } else {
            Ok(TokenKind::Variable)
        }
    }

    fn scan_number(&mut self) -> Result<TokenKind, &'static str> {
        while self.peek_char().is_ascii_digit() {
            self.advance_char();
        }

        if self.peek_char() == b'.' {
            while self.current != 0 && self.peek_char().is_ascii_digit() {
                self.reverse_char();
            }

            return self.scan_float();
        }

        let value = self.lexeme()?;
        if let Ok(i) = value.parse::<u64>() {
            Ok(TokenKind::Integer(i))
        } else {
            Err("invalid number")
        }
    }

    fn scan_float(&mut self) -> Result<TokenKind, &'static str> {
        while self.peek_char().is_ascii_digit() {
            self.advance_char();
        }

        if self.peek_char() == b'.' {
            self.advance_char();
            while self.peek_char().is_ascii_digit() {
                self.advance_char();
            }
        } else {
            return Err("idk lol");
        }

        let value = self.lexeme()?;
        if let Ok(f) = value.parse::<f64>() {
            Ok(TokenKind::Float(f))
        } else {
            Err("invalid number")
        }
    }

    fn slurp_whitespace(&mut self) {
        while self.peek_char().is_ascii_whitespace() {
            self.advance_char();
        }
    }

    fn add_token(&mut self, kind: TokenKind) -> Result<(), &'static str> {
        self.tokens.push_back(Token::new(kind, self.lexeme()?));
        Ok(())
    }

    fn advance_char(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn reverse_char(&mut self) -> u8 {
        self.current -= 1;
        self.source[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek_char(&mut self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn lexeme(&self) -> Result<&'a str, &'static str> {
        core::str::from_utf8(&self.source[self.start..self.current]).map_err(|_| "invalid utf-8")
    }
}

fn is_non_identifier(c: u8) -> bool {
    c.is_ascii_whitespace()
        || c == 0x00
        || c == b'('
        || c == b')'
        || c == b'-'
        || c == b'+'
        || c == b'*'
        || c == b'/'
        || c == b'^'
        || c == b'%'
        || c == b','
        || c == b'!'
        || c == b'>'
        || c == b'<'
        || c == b'='
        || c == b','
}

fn to_keyword(token: &str) -> Option<TokenKind> {
    match token {
        "sin" | "cos" | "tan" | "csc" | "sec" | "cot" | "sigma" | "ln" | "log" => {
            Some(TokenKind::BuiltinFunction)
        }
        "theta" | "dx" | "dy" | "dtheta" => Some(TokenKind::SpecialVariable),
        _ => None,
    }
}
