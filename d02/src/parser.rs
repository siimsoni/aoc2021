use btoi::btoi;
use std::io::BufRead;
use crate::tokenizer::{Tokenizer, TokenKind};
use std::convert::TryFrom;

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
            _ => Err("unknown identifier")
        }
    }
}

pub fn parse<R>(mut reader:R) -> Vec<Stmt>
    where R: BufRead {
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
    let mut token_iter = tokenizer.tokens.iter();
    let mut res = Vec::new();
    let mut kind: Option<StmtKind> = None;
    let mut value: Option<i16> = None;
    let mut pos = 0;
    for token in &mut token_iter {
        match token.kind {
            TokenKind::Lit => value = Some(btoi(&buffer[pos..pos+token.len]).expect("int")),
            TokenKind::Ident => kind = StmtKind::try_from(&buffer[pos..pos+token.len]).ok(),
            TokenKind::Newline => {
                if let Some(command) = make_stmt(kind, value) {
                    res.push(command);
                }
                kind = None;
                value = None;
            }
            _ => ()
        }
        pos += token.len;
    }
    if let Some(command) = make_stmt(kind, value) {
        res.push(command);
    }
    res
}

fn make_stmt(kw: Option<StmtKind>, val: Option<i16>) -> Option<Stmt> {
    val.and_then(|val| kw.and_then(|kw| Some((kw, val))))
}
