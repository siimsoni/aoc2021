use bit_vec::BitVec;
use std::io::BufRead;
use std::slice::Iter;

pub fn parse<R>(mut reader: R) -> (BitVec, BitVec, usize)
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

    let mut algorithm = BitVec::with_capacity(2);
    algorithm = eat_line(algorithm, &mut iter);

    let size_hint = iter.size_hint().0;
    let mut input_image = BitVec::with_capacity(if size_hint > 0 { size_hint } else { 2 });
    while input_image.len() == 0 {
        input_image = eat_line(input_image, &mut iter);
    }
    let width = input_image.len();
    let mut length = 0;
    loop {
        input_image = eat_line(input_image, &mut iter);
        if input_image.len() == length {
            break;
        } else {
            length = input_image.len();
        }
    }

    (algorithm, input_image, width)
}

fn eat_line(mut bit_vec: BitVec, iter: &mut Iter<u8>) -> BitVec {
    while let Some(byte) = iter.next() {
        match byte {
            b'\n' => {
                break;
            }
            b'.' => {
                if bit_vec.capacity() == bit_vec.len() {
                    bit_vec.reserve(bit_vec.capacity());
                }
                bit_vec.grow(1, false);
            }
            b'#' => {
                if bit_vec.capacity() == bit_vec.len() {
                    bit_vec.reserve(bit_vec.capacity());
                }
                bit_vec.grow(1, true);
            }
            _ => (),
        }
    }
    bit_vec
}
