use crate::tokenizer::{TokenKind, Tokenizer};
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<Box<[u8]>>
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
    while let Some(token) = iter.next() {
        if token.kind == TokenKind::Delimiter {
            result.push(Box::from(&buffer[pos..pos + token.len]));
        }
        pos += token.len;
    }
    result
}
