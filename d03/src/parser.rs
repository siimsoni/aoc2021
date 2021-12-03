use crate::tokenizer::{TokenKind, Tokenizer};
use std::io::BufRead;

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
    let mut res = Vec::new();

    let mut pos = 0;
    for token in tokenizer.tokens.iter() {
        match token.kind {
            TokenKind::Lit => {
                if let Some(int) = parse_u8(&buffer[pos..pos + token.len]) {
                    res.push(int);
                }
            }
            _ => (),
        }
        pos += token.len;
    }

    res
}

fn parse_u8(buffer: &[u8]) -> Option<usize> {
    let mut result = 0;
    let last = buffer.len() - 1;
    for n in 0..=last {
        if buffer[last - n] == b'1' {
            result += 1 << n;
        }
    }
    Some(result)
}
