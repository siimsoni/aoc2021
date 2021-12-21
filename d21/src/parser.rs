use crate::tokenizer::{Token, Tokenizer};
use btoi::btoi;
use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> Option<(usize, usize)>
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
    let mut pos = 0;

    consume_starting_position(&mut iter, &buffer, &mut pos).and_then(|p1| {
        consume_starting_position(&mut iter, &buffer, &mut pos).and_then(|p2| Some((p1, p2)))
    })
}

fn consume_starting_position(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Option<usize> {
    // "Player "...
    if let Some(token) = iter.next() {
        *pos += token.len;
    }
    // Player number...
    if let Some(token) = iter.next() {
        *pos += token.len;
    }
    // " starting position: "...
    if let Some(token) = iter.next() {
        *pos += token.len;
    }
    if let Some(token) = iter.next() {
        let result = btoi(&buffer[*pos..*pos + token.len]);
        *pos += token.len;
        return result.ok();
    }
    None
}
