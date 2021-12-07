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
            kind: TokenKind::Other,
            len: 0,
        }
    }

    pub fn tokenize(&mut self, iter: &mut std::slice::Iter<u8>) {
        for c in iter {
            let kind = match c {
                b'0'..=b'9' => TokenKind::Integer,
                _ => TokenKind::Other,
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
