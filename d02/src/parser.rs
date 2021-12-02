use btoi::btoi;
use std::io::BufRead;
use crate::tokenizer::{Tokenizer, TokenKind};
use std::convert::TryFrom;

pub type Command = (Keyword, i16);

#[derive(Clone, Debug)]
pub enum Keyword {
    Up,
    Down,
    Forward,
}

impl TryFrom<&[u8]> for Keyword {
    type Error = &'static str;

    fn try_from(str: &[u8]) -> Result<Self, Self::Error> {
        match str {
            b"up" => Ok(Keyword::Up),
            b"down" => Ok(Keyword::Down),
            b"forward" => Ok(Keyword::Forward),
            _ => Err("unknown identifier")
        }
    }
}

pub fn parse<R>(mut reader:R) -> Vec<Command>
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
    let mut keyword: Option<Keyword> = None;
    let mut value: Option<i16> = None;
    let mut pos = 0;
    for token in &mut token_iter {
        match token.kind {
            TokenKind::Integer => {
                value = Some(btoi(&buffer[pos..pos+token.len]).expect("int"))
            },
            TokenKind::Ident => {
                keyword = Keyword::try_from(&buffer[pos..pos+token.len]).ok()
            },
            TokenKind::Newline => {
                if let Some(command) = parse_ident(keyword, value) {
                    res.push(command);
                }
                keyword = None;
                value = None;
            }
            _ => ()
        }
        pos += token.len;
    }
    if let Some(command) = parse_ident(keyword, value) {
        res.push(command);
    }
    res
}

fn parse_ident(keyword: Option<Keyword>, value: Option<i16>) -> Option<Command> {
    if let Some(value) = value {
        if let Some(keyword) = keyword {
            return Some((keyword, value));
        }
    }
    None
}
