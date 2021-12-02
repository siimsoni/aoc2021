use crate::tokenizer::{TokenKind, Tokenizer};
use btoi::btoi;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<u16>
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
    let mut token_iter = tokenizer.tokens.iter();
    let mut pos = 0;
    let mut res = Vec::new();
    for token in &mut token_iter {
        if token.kind == TokenKind::Integer {
            res.push(btoi(&buffer[pos..pos + token.len]).expect("int"));
        }
        pos += token.len;
    }
    res
}
