use crate::tokenizer::{Token, TokenKind, Tokenizer};
use btoi::btoi;
use std::io::BufRead;
use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Segment {
    pub start: Coordinates,
    pub end: Coordinates,
}

pub fn parse<R>(mut reader: R) -> Vec<usize>
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

    let mut iter = tokenizer.tokens.iter();
    let mut result = Vec::new();
    let mut pos: usize = 0;
    while let Ok(segment) = eat_int(&mut iter, buffer.as_slice(), &mut pos) {
        result.push(segment);
    }
    result
}

fn eat_int(iter: &mut Iter<Token>, buffer: &[u8], pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("end of input").and_then(|token| {
        if token.kind != TokenKind::Integer {
            return Err("expected numeric coordinate");
        }
        let parsed = btoi::<usize>(&buffer[*pos..*pos + token.len]);
        *pos += token.len;
        // consume separator
        if let Some(token) = iter.next() {
            *pos += token.len;
        }
        parsed.or(Err("failed to parse coordinate"))
    })
}
