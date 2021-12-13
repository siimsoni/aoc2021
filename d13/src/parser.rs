use crate::tokenizer::{Token, TokenKind, Tokenizer};
use btoi::btoi;
use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> (Vec<(usize, usize)>, Vec<(char, usize)>)
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
    let mut points = Vec::new();
    let mut pos: usize = 0;
    while let Ok(coords) = eat_coordinates(&mut iter, &buffer, &mut pos) {
        points.push(coords);
        if let Ok(len) = eat_nl(&mut iter, &mut pos) {
            if len > 1 {
                break;
            }
        }
    }
    let mut instructions = Vec::new();
    while let Ok(instr) = eat_instructions(&mut iter, &buffer, &mut pos) {
        instructions.push(instr);
        eat_nl(&mut iter, &mut pos).ok();
    }

    (points, instructions)
}

fn eat_coordinates(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<(usize, usize), &'static str> {
    capture_int(iter, buffer, pos).and_then(|x| {
        eat_comma(iter, pos).and_then(|_| capture_int(iter, buffer, pos).and_then(|y| Ok((x, y))))
    })
}

fn eat_instructions(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<(char, usize), &'static str> {
    eat_literal(iter, pos)
        .and_then(|_| eat_space(iter, pos))
        .and_then(|_| eat_literal(iter, pos))
        .and_then(|_| eat_space(iter, pos))
        .and_then(|_| {
            capture_char(iter, buffer, pos).and_then(|c| {
                eat_eq(iter, pos)
                    .and_then(|_| capture_int(iter, buffer, pos).and_then(|val| Ok((c, val))))
            })
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

fn eat_space(iter: &mut Iter<Token>, pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        *pos += token.len;
        if token.kind != TokenKind::Space {
            return Err("expected space");
        }
        Ok(token.len)
    })
}

fn eat_comma(iter: &mut Iter<Token>, pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        *pos += token.len;
        if token.kind != TokenKind::Comma {
            return Err("expected comma");
        }
        Ok(token.len)
    })
}

fn eat_eq(iter: &mut Iter<Token>, pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        *pos += token.len;
        if token.kind != TokenKind::Eq {
            return Err("expected =");
        }
        Ok(token.len)
    })
}

fn capture_int(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        if token.kind != TokenKind::Integer {
            *pos += token.len;
            return Err("expected int");
        }
        let parsed = btoi::<usize>(&buffer[*pos..*pos + token.len]).or(Err("failed to parse int"));
        *pos += token.len;
        parsed
    })
}

fn eat_literal(iter: &mut Iter<Token>, pos: &mut usize) -> Result<usize, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        *pos += token.len;
        if token.kind != TokenKind::Literal {
            return Err("expected nl");
        }
        Ok(token.len)
    })
}

fn capture_char(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<char, &'static str> {
    iter.next().ok_or("eof").and_then(|token| {
        if token.kind != TokenKind::Literal {
            *pos += token.len;
            return Err("expected nl");
        }
        if token.len != 1 {
            *pos += token.len;
            return Err("expected len 1");
        }
        let result = buffer[*pos] as char;
        *pos += token.len;
        Ok(result)
    })
}
