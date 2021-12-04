use crate::tokenizer::{TokenKind, Tokenizer};
use btoi::btoi;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>)
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

    let mut drawed_numbers = Vec::new();
    for token in &mut iter {
        match token.kind {
            TokenKind::Integer => {
                if let Ok(int) = btoi::<usize>(&buffer[pos..pos + token.len]) {
                    drawed_numbers.push(int);
                }
            }
            TokenKind::Newline => {
                pos += token.len;
                break
            },
            _ => (),
        }
        pos += token.len;
    }

    let mut boards = Vec::new();
    let mut board = Vec::new();
    let mut row = 0;
    let mut col = 0;

    for token in &mut iter {
        match token.kind {
            TokenKind::Integer => {
                if let Ok(int) = btoi::<usize>(&buffer[pos..pos + token.len]) {
                    board.push((int, row, col));
                    col += 1;
                }
            },
            TokenKind::Newline => {
                if token.len > 1 {
                    boards.push(board.clone());
                    board = Vec::new();
                    row = 0;
                } else {
                    row += 1;
                }
                col = 0;
            },
            _ => (),
        }
        pos += token.len;
    }

    if board.len() > 0 {
        boards.push(board.clone());
    }

    (drawed_numbers, boards)
}
