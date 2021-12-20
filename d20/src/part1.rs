use bit_vec::BitVec;

pub fn solve(input: &(BitVec, BitVec, usize)) -> usize {
    match input {
        (algorithm, in_img, width) => {
            let (in_img, background, width) = enhance(algorithm, in_img.clone(), false, *width);
            let (in_img, _, _) = enhance(algorithm, in_img, background, width);
            in_img.iter().filter(|x| *x).count()
        }
    }
}

fn enhance(
    algorithm: &BitVec,
    in_img: BitVec,
    background: bool,
    width: usize,
) -> (BitVec, bool, usize) {
    let height = in_img.len() / width;
    let new_len = in_img.len() + (width * 4) + 4;
    let new_width = width + 2;
    let mut result = BitVec::with_capacity(new_len);
    result.grow(new_len, background);

    let mut col = 0;
    let mut row = 0;

    for _ in 0..result.len() {
        let mut idx = 0;

        if row > 1 {
            // top left
            if col > 1 {
                if in_img[(row - 2) * width + (col - 2)] {
                    idx |= 1 << 8;
                }
            } else if background {
                idx |= 1 << 8;
            }

            // top mid
            if col > 0 && col <= width {
                if in_img[(row - 2) * width + (col - 1)] {
                    idx |= 1 << 7;
                }
            } else if background {
                idx |= 1 << 7;
            }

            // top right
            if col < width {
                if in_img[(row - 2) * width + col] {
                    idx |= 1 << 6;
                }
            } else if background {
                idx |= 1 << 6;
            }
        } else {
            if background {
                idx |= 1 << 8;
                idx |= 1 << 7;
                idx |= 1 << 6;
            }
        }
        if row > 0 && row <= height {
            // left
            if col > 1 {
                if in_img[(row - 1) * width + (col - 2)] {
                    idx |= 1 << 5;
                }
            } else if background {
                idx |= 1 << 5;
            }

            // mid
            if col > 0 && col <= width {
                if in_img[(row - 1) * width + (col - 1)] {
                    idx |= 1 << 4;
                }
            } else if background {
                idx |= 1 << 4;
            }

            // right
            if col < width {
                if in_img[(row - 1) * width + col] {
                    idx |= 1 << 3;
                }
            } else if background {
                idx |= 1 << 3;
            }
        } else {
            if background {
                idx |= 1 << 5;
                idx |= 1 << 4;
                idx |= 1 << 3;
            }
        }
        if row < height {
            // bottom left
            if col > 1 {
                if in_img[row * width + (col - 2)] {
                    idx |= 1 << 2;
                }
            } else if background {
                idx |= 1 << 2;
            }

            // bottom mid
            if col > 0 && col <= width {
                if in_img[row * width + (col - 1)] {
                    idx |= 1 << 1;
                }
            } else if background {
                idx |= 1 << 1;
            }

            // bottom right
            if col < width {
                if in_img[row * width + col] {
                    idx |= 1;
                }
            } else if background {
                idx |= 1;
            }
        } else {
            if background {
                idx |= 1 << 2;
                idx |= 1 << 1;
                idx |= 1;
            }
        }

        result.set(row * new_width + col, algorithm[idx]);
        col += 1;
        if col == new_width {
            col = 0;
            row += 1;
        }
    }
    (
        result,
        if background {
            algorithm[511]
        } else {
            algorithm[0]
        },
        new_width,
    )
}
