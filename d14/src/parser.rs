use crate::tokenizer::{Token, TokenKind, Tokenizer};
use std::io::BufRead;
use std::slice::Iter;

pub type Transformation = ((char, char), char);

pub fn parse<R>(mut reader: R) -> (Vec<char>, Vec<Transformation>)
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
    let mut pos: usize = 0;

    if let Ok(pairs) = capture_chars(&mut iter, &buffer, &mut pos) {
        eat_nl(&mut iter, &mut pos).ok();
        let mut transformations = Vec::new();
        while let Ok(transformation) = capture_transformations(&mut iter, &buffer, &mut pos) {
            transformations.push(transformation);
            eat_nl(&mut iter, &mut pos).ok();
        }
        return (pairs, transformations);
    }

    return (Vec::new(), Vec::new());
}

fn capture_chars(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<Vec<char>, &'static str> {
    capture_string(iter, buffer, pos).and_then(|str| {
        return Ok(str.iter().map(|byte| *byte as char).collect());
    })
}

fn capture_transformations(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<Transformation, &'static str> {
    capture_string(iter, buffer, pos).and_then(|pair| {
        if pair.len() != 2 {
            return Err("pair len must be 2");
        }
        eat_arrow(iter, pos).and_then(|_| {
            capture_string(iter, buffer, pos)
                .and_then(|c| Ok(((pair[0] as char, pair[1] as char), c[0] as char)))
        })
    })
}

fn capture_string(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<Box<[u8]>, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        if token.kind != TokenKind::String {
            *pos += token.len;
            return Err("expected str");
        }
        let parsed = (&buffer[*pos..*pos + token.len])
            .to_vec()
            .into_boxed_slice();
        *pos += token.len;
        Ok(parsed)
    })
}

fn eat_nl(iter: &mut Iter<Token>, pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        *pos += token.len;
        if token.kind != TokenKind::Newline {
            return Err("expected nl");
        }
        Ok(token.len)
    })
}

fn eat_arrow(iter: &mut Iter<Token>, pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        *pos += token.len;
        if token.kind != TokenKind::Arrow {
            return Err("expected arrow");
        }
        Ok(token.len)
    })
}
