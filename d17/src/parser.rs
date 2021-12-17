use crate::tokenizer::{Token, TokenKind, Tokenizer};
use btoi::btoi;
use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> Result<(i32, i32, i32, i32), &'static str>
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
    consume(&mut iter, &buffer, &mut pos)
}

fn consume(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<(i32, i32, i32, i32), &'static str> {
    eat_header(iter, pos).and_then(|_| {
        consume_axis(iter, &buffer, pos).and_then(|x| match x {
            (char, min_x, max_x) => match char {
                'x' => expect_fixed(iter, pos, TokenKind::Comma, 1)
                    .and_then(|_| expect_fixed(iter, pos, TokenKind::Whitespace, 1))
                    .and_then(|_| {
                        consume_axis(iter, &buffer, pos).and_then(|y| match y {
                            (char, min_y, max_y) => match char {
                                'y' => Ok((min_x, max_x, min_y, max_y)),
                                _ => Err("unexpected character"),
                            },
                        })
                    }),
                _ => Err("unexpected coordinate"),
            },
        })
    })
}

fn eat_header(iter: &mut Iter<Token>, pos: &mut usize) -> Result<(), &'static str> {
    expect_fixed(iter, pos, TokenKind::String, 6)
        .and_then(|_| expect_fixed(iter, pos, TokenKind::Whitespace, 1))
        .and_then(|_| expect_fixed(iter, pos, TokenKind::String, 4))
        .and_then(|_| expect_fixed(iter, pos, TokenKind::Colon, 1))
        .and_then(|_| expect_fixed(iter, pos, TokenKind::Whitespace, 1))
}

fn consume_axis(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<(char, i32, i32), &'static str> {
    consume_kind(iter, buffer, pos, TokenKind::String).and_then(|axis| {
        expect_fixed(iter, pos, TokenKind::Eq, 1).and_then(|_| {
            consume_int(iter, buffer, pos).and_then(|min| {
                expect_fixed(iter, pos, TokenKind::Range, 2).and_then(|_| {
                    consume_int(iter, buffer, pos).and_then(|max| Ok((axis[0] as char, min, max)))
                })
            })
        })
    })
}

fn consume_int(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<i32, &'static str> {
    consume_kind(iter, buffer, pos, TokenKind::SignedInt)
        .and_then(|bytes| btoi(&bytes).or(Err("failed to parse int")))
}

fn consume_kind(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
    expected_kind: TokenKind,
) -> Result<Box<[u8]>, &'static str> {
    iter.next()
        .ok_or("unexpected end of input")
        .and_then(|token| {
            if token.kind != expected_kind {
                *pos += token.len;
                return Err("unexpected token kind");
            }
            let result = &buffer[*pos..*pos + token.len];
            *pos += token.len;
            Ok(result.into())
        })
}

fn expect_fixed(
    iter: &mut Iter<Token>,
    pos: &mut usize,
    expected_kind: TokenKind,
    expected_len: usize,
) -> Result<(), &'static str> {
    iter.next()
        .ok_or("unexpected end of input")
        .and_then(|token| {
            *pos += token.len;
            if token.kind != expected_kind {
                return Err("unexpected token kind");
            }
            if token.len != expected_len {
                return Err("unexpected token len");
            }
            Ok(())
        })
}
