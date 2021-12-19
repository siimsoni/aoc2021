#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Integer,
    Comma,
    Newline,
    Header,
    Other,
}

pub struct Tokenizer {
    pub tokens: Vec<Token>,
    kind: TokenKind,
    len: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            tokens: Vec::new(),
            kind: TokenKind::Header,
            len: 0,
        }
    }

    pub fn tokenize(&mut self, iter: &mut std::slice::Iter<u8>) {
        for c in iter {
            let kind = if let TokenKind::Header = self.kind {
                match *c {
                    b'\n' => TokenKind::Newline,
                    _ => TokenKind::Header,
                }
            } else {
                match *c {
                    b'0'..=b'9' => TokenKind::Integer,
                    b',' => TokenKind::Comma,
                    b'\n' => TokenKind::Newline,
                    b'-' => {
                        if let TokenKind::Newline = self.kind {
                            if self.len > 1 {
                                TokenKind::Header
                            } else {
                                TokenKind::Integer
                            }
                        } else {
                            TokenKind::Integer
                        }
                    }
                    _ => TokenKind::Other,
                }
            };
            if kind == self.kind {
                self.len += 1;
            } else {
                self.flush();
                self.kind = kind;
                self.len = 1;
            }
        }
    }

    pub fn flush(&mut self) {
        if self.len > 0 {
            self.tokens.push(Token::new(self.kind, self.len))
        }
    }
}
