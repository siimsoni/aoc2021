use bit_vec::BitVec;
use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> Vec<BitVec>
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
    let mut transmission = Vec::new();
    while let Ok(byte) = eat_int(&mut iter) {
        if let Some(int) = byte {
            transmission.push(int);
        } else {
            let capacity = transmission.capacity();
            result.push(transmission_to_bv(transmission));
            transmission = Vec::with_capacity(capacity);
        }
    }
    if transmission.len() > 0 {
        result.push(transmission_to_bv(transmission));
    }

    result
}

fn transmission_to_bv(transmission: Vec<u8>) -> BitVec {
    let mut bv = BitVec::from_elem(transmission.len() * 4, false);
    let mut pos = 0;
    for nibble in transmission {
        bv.set(pos, ((nibble >> 3) & 1) != 0);
        pos += 1;
        bv.set(pos, ((nibble >> 2) & 1) != 0);
        pos += 1;
        bv.set(pos, ((nibble >> 1) & 1) != 0);
        pos += 1;
        bv.set(pos, ((nibble >> 0) & 1) != 0);
        pos += 1;
    }
    bv
}

fn eat_int(iter: &mut Iter<u8>) -> Result<Option<u8>, &'static str> {
    iter.next()
        .ok_or("end of input")
        .and_then(|byte| match byte {
            b'0' => Ok(Some(0)),
            b'1' => Ok(Some(1)),
            b'2' => Ok(Some(2)),
            b'3' => Ok(Some(3)),
            b'4' => Ok(Some(4)),
            b'5' => Ok(Some(5)),
            b'6' => Ok(Some(6)),
            b'7' => Ok(Some(7)),
            b'8' => Ok(Some(8)),
            b'9' => Ok(Some(9)),
            b'A' => Ok(Some(10)),
            b'B' => Ok(Some(11)),
            b'C' => Ok(Some(12)),
            b'D' => Ok(Some(13)),
            b'E' => Ok(Some(14)),
            b'F' => Ok(Some(15)),
            b'\n' => Ok(None),
            _ => Err("unexpected character"),
        })
}
