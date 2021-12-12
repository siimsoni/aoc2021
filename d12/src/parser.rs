use crate::tokenizer::{Token, TokenKind, Tokenizer};
use std::io::BufRead;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum NodeSize {
    Small,
    Large,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NodeKind {
    Regular { id: Box<[u8]>, size: NodeSize },
    Start,
    End,
}

#[derive(Debug)]
pub struct NodeLink {
    pub from: NodeKind,
    pub to: NodeKind,
}

pub fn parse<R>(mut reader: R) -> Vec<NodeLink>
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
    while let Ok(path) = eat_link(&mut iter, buffer.as_slice(), &mut pos) {
        result.push(path);
    }
    result
}

fn eat_link(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<NodeLink, &'static str> {
    match eat_node(iter, buffer, pos) {
        Ok(from) => {
            if let Some(next) = iter.next() {
                *pos += next.len;
                if next.kind != TokenKind::Dash {
                    return Err("unexpected character when expecting dash");
                }
            } else {
                return Err("unexpected end of input");
            }
            if let Ok(to) = eat_node(iter, buffer, pos) {
                if let Some(next) = iter.next() {
                    *pos += next.len;
                    if next.kind != TokenKind::Newline {
                        return Err("unexpected token when expecting newline");
                    }
                }
                Ok(NodeLink { from, to })
            } else {
                return Err("unexpected end of input");
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
}

fn eat_node(
    iter: &mut Iter<Token>,
    buffer: &[u8],
    pos: &mut usize,
) -> Result<NodeKind, &'static str> {
    iter.next().ok_or("end of input").and_then(|token| {
        let result;
        if token.kind == TokenKind::String {
            let id = &buffer[*pos..*pos + token.len];
            result = match id {
                b"start" => Ok(NodeKind::Start),
                b"end" => Ok(NodeKind::End),
                _ => match id[0] {
                    b'a'..=b'z' => Ok(NodeKind::Regular {
                        id: Box::from(id),
                        size: NodeSize::Small,
                    }),
                    b'A'..=b'Z' => Ok(NodeKind::Regular {
                        id: Box::from(id),
                        size: NodeSize::Large,
                    }),
                    _ => Err("unexpected id"),
                },
            };
        } else {
            result = Err("unexpected token")
        }
        *pos += token.len;
        result
    })
}
