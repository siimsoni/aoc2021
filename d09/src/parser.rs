use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> (Vec<u8>, usize)
where
    R: BufRead,
{
    let mut buffer = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        buffer.extend_from_slice(&page[..page_len]);
    }

    let mut iter = buffer.iter();
    let mut result = Vec::new();
    while let Ok(int) = eat_int(&mut iter) {
        result.push(int);
    }
    let width = result.len();
    loop {
        for _ in 0..width {
            if let Ok(int) = eat_int(&mut iter) {
                result.push(int);
            }
        }
        if iter.next().is_none() {
            break;
        }
    }
    (result, width)
}

fn eat_int(iter: &mut Iter<u8>) -> Result<u8, &'static str> {
    iter.next().ok_or("end of input").and_then(|byte| {
        let result = match byte {
            b'1' => Ok(1),
            b'2' => Ok(2),
            b'3' => Ok(3),
            b'4' => Ok(4),
            b'5' => Ok(5),
            b'6' => Ok(6),
            b'7' => Ok(7),
            b'8' => Ok(8),
            b'9' => Ok(9),
            b'0' => Ok(0),
            _ => Err("unexpected character")
        };
        result
    })
}
