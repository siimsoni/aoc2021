use crate::tokenizer::{TokenKind, Tokenizer};
use btoi::btoi;
use std::convert::TryFrom;
use std::io::BufRead;

pub type Stmt = (StmtKind, i16);

#[derive(Clone, Debug)]
pub enum StmtKind {
    Up,
    Down,
    Forward,
}

impl TryFrom<&[u8]> for StmtKind {
    type Error = &'static str;

    fn try_from(str: &[u8]) -> Result<Self, Self::Error> {
        match str {
            b"up" => Ok(StmtKind::Up),
            b"down" => Ok(StmtKind::Down),
            b"forward" => Ok(StmtKind::Forward),
            _ => Err("unknown identifier"),
        }
    }
}

pub fn parse<R>(mut reader: R) -> Vec<Stmt>
where
    R: BufRead,
{
    let mut buffer = Vec::new();
    let mut tokenizer = Tokenizer::new();
    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        tokenizer.tokenize(&mut (page[..page_len].iter()));
        buffer.extend_from_slice(&page[..page_len]);
    }
    tokenizer.flush();
    let mut res = Vec::new();
    let mut kind: &[u8] = &[];
    let mut value: &[u8] = &[];
    let mut pos = 0;
    for token in tokenizer.tokens.iter() {
        match token.kind {
            TokenKind::Lit => value = &buffer[pos..pos + token.len],
            TokenKind::Ident => kind = &buffer[pos..pos + token.len],
            TokenKind::Newline => {
                if let Some(command) = make_stmt(kind, value) {
                    res.push(command);
                }
                kind = &[];
                value = &[];
            }
            _ => (),
        }
        pos += token.len;
    }
    if let Some(command) = make_stmt(kind, value) {
        res.push(command);
    }
    res
}

fn make_stmt(kw: &[u8], val: &[u8]) -> Option<Stmt> {
    StmtKind::try_from(kw)
        .ok()
        .and_then(|kw| btoi(val).ok().and_then(|val: i16| Some((kw, val))))
}
