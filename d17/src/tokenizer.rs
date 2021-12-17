#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    String,
    Colon,
    SignedInt,
    Eq,
    Range,
    Comma,
    Whitespace,
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
            kind: TokenKind::String,
            len: 0,
        }
    }

    pub fn tokenize(&mut self, iter: &mut std::slice::Iter<u8>) {
        for c in iter {
            let kind = match c {
                b'a'..=b'z' => TokenKind::String,
                b':' => TokenKind::Colon,
                b'-' | b'0'..=b'9' => TokenKind::SignedInt,
                b'=' => TokenKind::Eq,
                b'.' => TokenKind::Range,
                b',' => TokenKind::Comma,
                _ => TokenKind::Whitespace,
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

#[cfg(test)]
mod tests {
    use crate::tokenizer::{TokenKind, Tokenizer};

    #[test]
    fn tokenize_example() {
        let input = "target area: x=20..30, y=-10..-5".as_bytes();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(&mut input.iter());
        tokenizer.flush();
        let mut tokens = tokenizer.tokens;
        assert_eq!(tokens.len(), 17);
        let token = tokens.get(0).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::String, 6));
        let token = tokens.get(1).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Whitespace, 1));
        let token = tokens.get(2).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::String, 4));
        let token = tokens.get(3).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Colon, 1));
        let token = tokens.get(4).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Whitespace, 1));
        let token = tokens.get(5).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::String, 1));
        let token = tokens.get(6).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Eq, 1));
        let token = tokens.get(7).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::SignedInt, 2));
        let token = tokens.get(8).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Range, 2));
        let token = tokens.get(9).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::SignedInt, 2));
        let token = tokens.get(10).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Comma, 1));
        let token = tokens.get(11).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Whitespace, 1));
        let token = tokens.get(12).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::String, 1));
        let token = tokens.get(13).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Eq, 1));
        let token = tokens.get(14).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::SignedInt, 3));
        let token = tokens.get(15).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::Range, 2));
        let token = tokens.get(16).unwrap();
        assert_eq!((token.kind, token.len), (TokenKind::SignedInt, 2));
    }
}
