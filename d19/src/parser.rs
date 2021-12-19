use crate::tokenizer::{TokenKind, Tokenizer};
use btoi::btoi;
use fxhash::FxHashSet;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<FxHashSet<[i32; 3]>>
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

    let mut result = Vec::new();
    let mut scanner: FxHashSet<[i32; 3]> = FxHashSet::default();
    let mut current_pos = 0;
    let mut current_val = [0, 0, 0];
    let mut pos = 0;
    for token in tokenizer.tokens {
        match token.kind {
            TokenKind::Header => {
                if scanner.len() > 0 {
                    result.push(scanner.clone());
                    scanner.clear();
                }
            }
            TokenKind::Integer => {
                current_val[current_pos] = btoi(&buffer[pos..(pos + token.len)]).unwrap()
            }
            TokenKind::Comma => current_pos += 1,
            TokenKind::Newline => {
                if current_pos == 2 {
                    scanner.insert(current_val);
                }
                current_pos = 0;
            }
            _ => (),
        }
        pos += token.len;
    }
    if current_pos == 2 {
        scanner.insert(current_val);
    }
    if scanner.len() > 0 {
        result.push(scanner);
    }
    result
}
