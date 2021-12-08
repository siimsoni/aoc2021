use crate::tokenizer::{Token, TokenKind, Tokenizer};
use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> Vec<(Vec<u8>, Vec<u8>)>
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
    while let Ok(entry) = eat_entry(&mut iter, buffer.as_slice(), &mut pos) {
        result.push(entry);
    }
    result
}

fn eat_entry(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let mut patterns = Vec::new();
    loop {
        let result = eat_pattern(iter, buffer, pos);
        if let Ok(pattern) = result {
            if let Some(pattern) = pattern {
                patterns.push(pattern);
            }
        } else {
            break;
        }
        // consume whitespace
        if let Some(token) = iter.next() {
            *pos += token.len;
        }
    }

    if patterns.len() == 0 {
        return Err("no patterns");
    }

    let mut output = Vec::new();

    // consume whitespace
    if let Some(token) = iter.next() {
        *pos += token.len;
    }

    loop {
        let result = eat_pattern(iter, buffer, pos);
        if let Ok(pattern) = result {
            if let Some(pattern) = pattern {
                output.push(pattern);
            }
        } else {
            break;
        }
        // consume whitespace
        if let Some(token) = iter.next() {
            *pos += token.len;
            if token.kind == TokenKind::Newline {
                break;
            }
        }
    }

    if output.len() == 0 {
        return Err("no output");
    }

    return Ok((patterns, output));
}

fn eat_pattern(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<Option<u8>, &'static str> {
    iter.next().ok_or("end of input").and_then(|token| {
        if token.kind != TokenKind::Pattern {
            *pos += token.len;
            return Err("pattern not found");
        }
        let mut pattern = 0;
        for n in *pos..*pos + token.len {
            pattern += 1 << (buffer[n] - b'a');
        }
        *pos += token.len;
        let result = Ok(Some(pattern));
        result
    })
}
