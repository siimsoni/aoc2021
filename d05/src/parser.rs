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

pub fn parse<R>(mut reader: R) -> Vec<Segment>
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
    while let Ok(segment) = eat_segment(&mut iter, buffer.as_slice(), &mut pos) {
        result.push(segment);
    }
    result
}

fn eat_segment(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<Segment, &'static str> {
    let result = eat_coords(iter, buffer, pos).and_then(|start| {
        if let Some(token) = iter.next() {
            if token.kind != TokenKind::Arrow {
                return Err("expected arrow after coord");
            }
            *pos += token.len;
        }
        eat_coords(iter, buffer, pos).and_then(|end| Ok(Segment { start, end }))
    });
    // consume the newline
    if let Some(token) = iter.next() {
        if token.kind != TokenKind::Newline {
            return Err("expected newline after segment");
        }
        *pos += token.len;
    }
    result
}

fn eat_coords(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<Coordinates, &'static str> {
    eat_int(iter, buffer, pos).and_then(|x| {
        if let Some(token) = iter.next() {
            if token.kind != TokenKind::Comma {
                return Err("expected comma between coords");
            }
            *pos += token.len;
        }
        eat_int(iter, buffer, pos).and_then(|y| Ok(Coordinates { x, y }))
    })
}

fn eat_int(iter: &mut Iter<Token>, buffer: &[u8], pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("end of input").and_then(|token| {
        if token.kind != TokenKind::Integer {
            return Err("expected numeric coordinate");
        }
        let parsed = btoi::<usize>(&buffer[*pos..*pos + token.len]);
        *pos += token.len;
        parsed.or(Err("failed to parse coordinate"))
    })
}
