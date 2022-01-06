use std::io::{self, Read};

struct Chunkable<I: Iterator<Item = io::Result<u8>>> {
    iter: I,
    size: usize,
}

impl<I: Iterator<Item = io::Result<u8>>> Chunkable<I> {
    pub fn new(iter: I, size: usize) -> Chunkable<I> {
        Chunkable { iter, size }
    }
}

impl<I: Iterator<Item = io::Result<u8>>> Iterator for Chunkable<I> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = Vec::with_capacity(self.size);
        for i in 0..self.size {
            match self.iter.next() {
                Some(Ok(elem)) => vec.push(elem),
                _ => {
                    if i == 0 {
                        return None;
                    }
                    break;
                }
            }
        }
        Some(vec)
    }
}

fn main() {
    let input = io::stdin();
    let mut input = Chunkable::new(input.lock().bytes(), 16);
    let mut index = 0;
    loop {
        match input.next() {
            None => break,
            Some(bytes) => {
                let ilen = bytes.len();
                let mut output = String::new();

                output.push_str(&format!("{:08x}", index));

                // 0..16 is necessary here
                #[allow(clippy::needless_range_loop)]
                for i in 0..16 {
                    if i % 8 == 0 {
                        output.push(' ');
                    }
                    output.push(' ');
                    if i >= ilen {
                        output.push_str("  ");
                    } else {
                        output.push_str(&format!("{:02x}", bytes[i]));
                    }
                }

                output.push_str("  |");
                for c in bytes.iter().map(|b| {
                    let b = char::from(*b);
                    if !(' '..='~').contains(&b) {
                        '.'
                    } else {
                        b
                    }
                }) {
                    output.push(c);
                }
                output.push('|');

                println!("{}", output);
                index += ilen;
            }
        }
    }
    if index > 0 {
        println!("{:08x}", index);
    }
}
