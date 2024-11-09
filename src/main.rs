use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, BufWriter, Stdout};
use std::{env, io};

static ARRAY: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub trait WriteChar {
    fn write_char(&mut self, c: char);
}

impl WriteChar for String {
    fn write_char(&mut self, c: char) {
        write!(self, "{}", c).unwrap();
    }
}

impl WriteChar for Stdout {
    fn write_char(&mut self, c: char) {
        use std::io::Write;
        write!(self, "{}", c).unwrap();
    }
}

impl WriteChar for BufWriter<File> {
    fn write_char(&mut self, c: char) {
        use std::io::Write;
        write!(self, "{}", c).unwrap();
    }
}

fn base64_bis2(mut my_buf: impl io::BufRead, write: &mut impl WriteChar, length: Option<u32>) {
    let debug = false;
    let mut calcul = Calcul {
        rest: 0,
        i: 0,
        fin: false,
        debug,
    };
    let size0:u32;
    if length.is_some() {
        size0= length.unwrap() as u32;
    } else {
        size0=4096*32;
    }
    const SIZE:usize=4096*32;
    let mut buf = vec![0u8; size0 as usize];

    if false {
        while let Ok(_) = my_buf.read_exact(&mut buf) {
            for byte_or_error in buf.iter() {
                let byte = byte_or_error;

                calcul.calcul(*byte, write);
            }
        }
    } else {
        for byte_or_error in my_buf.bytes() {
            let byte = byte_or_error.unwrap();

            calcul.calcul(byte, write);
        }
    }

    calcul.calcul_fin(write);
}

trait Calcul64 {
    fn calcul(&mut self, byte: u8, write: &mut impl WriteChar);
    fn calcul0(&mut self, byte: u8, write: &mut impl WriteChar, fin: bool);
    fn calcul_fin(&mut self, write: &mut impl WriteChar);
}

struct Calcul {
    rest: u8,
    i: i8,
    fin: bool,
    debug: bool,
}

impl Calcul64 for Calcul {
    fn calcul(&mut self, byte: u8, write: &mut impl WriteChar) {
        assert!(!self.fin, "x wasn't true!");
        self.calcul0(byte, write, false);
    }

    fn calcul0(&mut self, byte: u8, write: &mut impl WriteChar, fin: bool) {
        let debug = self.debug;
        if debug {
            println!(
                "* i={},byte={}/{:#08b},rest={}/{:#08b}",
                self.i, byte, byte, self.rest, self.rest
            );
        }

        if self.i == 0 {
            let a = byte >> 2;
            let b = byte & 0b11;
            self.rest = b;
            if debug {
                println!(
                    "a={}/{:#08b},b={}/{:#08b},rest={}/{:#08b}",
                    a, a, b, b, self.rest, self.rest
                );
            }
            let c = ARRAY[a as usize];
            if debug {
                println!("c={}", c);
            }
            write.write_char(c);
        } else if self.i == 1 {
            let a = byte >> 4;
            let b = byte & 0b1111;
            let x = (self.rest << 4) + a;
            self.rest = b;
            if debug {
                println!(
                    "a={}/{:#08b},b={}/{:#08b},x={}/{:#08b},rest={}/{:#08b}",
                    a, a, b, b, x, x, self.rest, self.rest
                );
            }
            let c = ARRAY[x as usize];
            if debug {
                println!("c={}", c);
            }
            write.write_char(c);
        } else if self.i == 2 {
            let a = byte >> 6;
            let b = byte & 0b111111;
            let x = (self.rest << 2) + a;
            self.rest = 0;
            if debug {
                println!(
                    "a={}/{:#08b},b={}/{:#08b},x={}/{:#08b},rest={}/{:#08b}",
                    a, a, b, b, x, x, self.rest, self.rest
                );
            }
            let c = ARRAY[x as usize];
            let c2 = ARRAY[b as usize];
            if debug {
                println!("c={},c2={}", c, c2);
            }
            write.write_char(c);
            if !fin {
                write.write_char(c2);
            }
        }
        self.i = (self.i + 1) % 3;
    }

    fn calcul_fin(&mut self, write: &mut impl WriteChar) {
        let debug = self.debug;
        if debug {
            println!("fin: i={}", self.i);
        }
        if self.i == 1 {
            self.calcul(0, write);
            write.write_char('=');
            write.write_char('=');
        } else if self.i == 2 {
            self.calcul0(0, write, true);
            write.write_char('=');
        }
        self.fin = true;
    }
}

fn main() {
    let mut input: Option<&str> = None;
    let mut output: Option<&str> = None;
    let args: Vec<String> = env::args().collect();
    let mut s: String = String::from("");
    let mut s2: String = String::from("");
    let mut s3: String = String::from("");
    let mut buffer_size: Option<u32> = None;

    for arg in args {
        if arg.starts_with("--input=") {
            arg[8..].clone_into(&mut s);
            input = Some(&s);
        } else if arg.starts_with("--output=") {
            arg[9..].clone_into(&mut s2);
            output = Some(&s2);
        } else if arg.starts_with("--bufferSize=") {
            arg[13..].clone_into(&mut s3);
            let my_int = s3.parse::<u32>().unwrap();
            buffer_size = Some(my_int);
        }
    }

    match input {
        Some(x) => {
            let open = File::open(x).unwrap();
            let my_buf: BufReader<File>;
            if buffer_size.is_none() {
                my_buf = BufReader::new(open);
            } else {
                my_buf = BufReader::with_capacity(buffer_size.unwrap() as usize, open);
            }
            match output {
                Some(y) => {
                    let f = File::create(y).unwrap();
                    let mut out: BufWriter<File>;
                    if buffer_size.is_none() {
                        out = BufWriter::new(f);
                    } else {
                        out = BufWriter::with_capacity(buffer_size.unwrap() as usize, f);
                    }
                    base64_bis2(my_buf, &mut out, buffer_size);
                }
                _ => {
                    let mut stdout = io::stdout();
                    base64_bis2(my_buf, &mut stdout, buffer_size);
                }
            }
        }
        _ => {
            let my_buf = BufReader::new(io::stdin());
            match output {
                Some(y) => {
                    let f = File::create(y).unwrap();
                    let mut out: BufWriter<File>;
                    if buffer_size.is_none() {
                        out = BufWriter::new(f);
                    } else {
                        out = BufWriter::with_capacity(buffer_size.unwrap() as usize, f);
                    }
                    base64_bis2(my_buf, &mut out, buffer_size);
                }
                _ => {
                    let mut stdout = io::stdout();
                    base64_bis2(my_buf, &mut stdout, buffer_size);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base64_bis() {
        assert_eq!(base64t_bis("aaa".as_bytes()), "YWFh");
        assert_eq!(base64t_bis("Hi!".as_bytes()), "SGkh");
        assert_eq!(base64t_bis("aaaaaa".as_bytes()), "YWFhYWFh");
        assert_eq!(base64t_bis("Salut".as_bytes()), "U2FsdXQ=");
        assert_eq!(base64t_bis("a".as_bytes()), "YQ==");
        assert_eq!(base64t_bis("light w".as_bytes()), "bGlnaHQgdw==");
        assert_eq!(base64t_bis("light wo".as_bytes()), "bGlnaHQgd28=");
        assert_eq!(base64t_bis("light wor".as_bytes()), "bGlnaHQgd29y");
        assert_eq!(base64t_bis("light work".as_bytes()), "bGlnaHQgd29yaw==");
        assert_eq!(base64t_bis("light work.".as_bytes()), "bGlnaHQgd29yay4=");

        // test 'a'
        assert_eq!(base64t_bis("a".as_bytes()), "YQ==");

        // test 'b'
        assert_eq!(base64t_bis("b".as_bytes()), "Yg==");

        // test 'aa'
        assert_eq!(base64t_bis("aa".as_bytes()), "YWE=");

        // test 'aaa'
        assert_eq!(base64t_bis("aaa".as_bytes()), "YWFh");

        // test 'aaaa'
        assert_eq!(base64t_bis("aaaa".as_bytes()), "YWFhYQ=="); //
    }

    fn base64t_bis(my_buf: impl io::BufRead) -> String {
        let mut s = String::new();
        base64_bis2(my_buf, &mut s, None);
        s
    }
}
