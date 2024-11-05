use std::fmt::Write;
use std::fs::File;
use std::{env, io};
use std::io::{BufReader, BufWriter, Stdout};

static ARRAY: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

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

fn get_values(byte: u8) -> char {
    assert!(byte < 63);

    return ARRAY[byte as usize];
}

fn affiche(byte: u8, write: &mut impl WriteChar) {
    assert!(byte < 63, "byte={}", byte);

    let c: char = get_values(byte);

    write.write_char(c);
}

fn create_vector(byte: u8) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];

    for i in 0..8 {
        let b = (byte & (1u8 << i)) > 0;
        result.push(b);
    }

    return result.iter().copied().rev().collect();
}

fn split(vect: &Vec<bool>, pos: i8) -> (Vec<bool>, Vec<bool>) {
    let pos2: usize = (pos) as usize;
    let a = vect[0..pos2].to_vec();
    let b = vect[pos2..vect.len()].to_vec();

    assert_eq!(vect.len(), a.len() + b.len());
    return (a, b);
}

fn to_number(vect: Vec<bool>) -> u8 {
    let mut res: u8 = 0;
    for b in vect {
        res <<= 1;
        if b {
            res += 1;
        }
    }
    return res;
}

fn base64_bis(my_buf: impl io::BufRead, write: &mut impl WriteChar) {
    let debug = true;
    let mut i = 0;
    let mut rest = 0;
    //let mut vect: Vec<char> = vec![];
    let mut taille_rest = 0;
    let mut calcul = Calcul { rest: 0, i: 0, fin: false, debug };
    for byte_or_error in my_buf.bytes() {
        let byte = byte_or_error.unwrap();

        calcul.calcul(byte, write);
        /*if debug {
            println!("* i={},byte={}/{:#08b},rest={}/{:#08b}", i, byte, byte, rest, rest);
        }

        if i == 0 {
            let a = byte >> 2;
            let b = byte & 0b11;
            rest = b;
            taille_rest=2;
            if debug {
                println!("a={}/{:#08b},b={}/{:#08b},rest={}/{:#08b}", a, a, b, b, rest, rest);
            }
            let c = ARRAY[a as usize];
            if debug {
                println!("c={}", c);
            }
            //vect.push(c);
            write.write_char(c);
        } else if i == 1 {
            let a = byte >> 4;
            let b = byte & 0b1111;
            let x = (rest << 4) + a;
            rest = b;
            taille_rest=4;
            if debug {
                println!("a={}/{:#08b},b={}/{:#08b},x={}/{:#08b},rest={}/{:#08b}",
                         a, a, b, b, x, x, rest, rest);
            }
            let c = ARRAY[x as usize];
            if debug {
                println!("c={}", c);
            }
            //vect.push(c);
            write.write_char(c);
        } else if i == 2 {
            let a = byte >> 6;
            let b = byte & 0b111111;
            let x = (rest << 2) + a;
            rest = 0;
            taille_rest=0;
            if debug {
                println!("a={}/{:#08b},b={}/{:#08b},x={}/{:#08b},rest={}/{:#08b}",
                         a, a, b, b, x, x, rest, rest);
            }
            let c = ARRAY[x as usize];
            let c2 = ARRAY[b as usize];
            if debug {
                println!("c={},c2={}", c, c2);
            }
            //vect.push(c);
            write.write_char(c);
            write.write_char(c2);
        }
        i = (i + 1) % 3;*/
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
        self.calcul0(byte, write, false);
    }

    fn calcul0(&mut self, byte: u8, write: &mut impl WriteChar, fin: bool) {
        let debug = self.debug;
        if debug {
            println!("* i={},byte={}/{:#08b},rest={}/{:#08b}", self.i, byte, byte, self.rest, self.rest);
        }

        if self.i == 0 {
            let a = byte >> 2;
            let b = byte & 0b11;
            self.rest = b;
            // taille_rest=2;
            if debug {
                println!("a={}/{:#08b},b={}/{:#08b},rest={}/{:#08b}", a, a, b, b, self.rest, self.rest);
            }
            let c = ARRAY[a as usize];
            if debug {
                println!("c={}", c);
            }
            //vect.push(c);
            write.write_char(c);
        } else if self.i == 1 {
            let a = byte >> 4;
            let b = byte & 0b1111;
            let x = (self.rest << 4) + a;
            self.rest = b;
            //taille_rest=4;
            if debug {
                println!("a={}/{:#08b},b={}/{:#08b},x={}/{:#08b},rest={}/{:#08b}",
                         a, a, b, b, x, x, self.rest, self.rest);
            }
            let c = ARRAY[x as usize];
            if debug {
                println!("c={}", c);
            }
            //vect.push(c);
            write.write_char(c);
        } else if self.i == 2 {
            let a = byte >> 6;
            let b = byte & 0b111111;
            let x = (self.rest << 2) + a;
            self.rest = 0;
            //taille_rest=0;
            if debug {
                println!("a={}/{:#08b},b={}/{:#08b},x={}/{:#08b},rest={}/{:#08b}",
                         a, a, b, b, x, x, self.rest, self.rest);
            }
            let c = ARRAY[x as usize];
            let c2 = ARRAY[b as usize];
            if debug {
                println!("c={},c2={}", c, c2);
            }
            //vect.push(c);
            write.write_char(c);
            if (!fin) {
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
        // while(self.i!=0){
        //     self.calcul(u8::try_from('=').unwrap(), write);
        // }
        if (self.i == 1) {
            self.calcul(0, write);
            write.write_char('=');
            write.write_char('=');
            //write.write_char('=');
        } else if (self.i == 2) {
            self.calcul0(0, write, true);
            write.write_char('=');
            // let x = self.rest ;
            // let c = ARRAY[x as usize];
            // write.write_char(c);
            // write.write_char('=');
            //write.write_char('=');
            //self.calcul(u8::try_from('=').unwrap(), write);
        }
        self.fin = true;
    }
}

fn base64(my_buf: impl io::BufRead, write: &mut impl WriteChar) {
    let debug = false;
    let mut v2 = vec![];
    let mut no = 0;
    let mut nb_affiche = 0;
    for byte_or_error in my_buf.bytes() {
        let byte = byte_or_error.unwrap();

        let v = create_vector(byte);

        v2 = [v2, v].concat();

        if v2.len() >= 24 {
            if debug {
                println!("no={}", no);
                println!("v2(len={})={:?}", v2.len(), v2);
            }
            let nb_affiche2: i32;
            let res: Option<Vec<bool>>;
            (_, nb_affiche2, res) = construit_resultat(debug, &v2, write);
            match res {
                Some(v) => v2 = v,
                _ => {}
            }
            nb_affiche += nb_affiche2;
            no += 1;
        }
    }


    if debug {
        println!("construction du resultat: v2={:?}", v2);
    }

    let mut termine = v2.len() == 0;
    while !termine {
        if debug {
            println!("no={}", no);
            println!("v2(len={})={:?}", v2.len(), v2);
        }
        let nb_affiche2: i32;
        let res: Option<Vec<bool>>;
        (termine, nb_affiche2, res) = construit_resultat(debug, &v2, write);
        match res {
            Some(v) => v2 = v,
            _ => {}
        }
        nb_affiche += nb_affiche2;
        no += 1;
    }
    if nb_affiche % 4 != 0 {
        for _ in (nb_affiche % 4)..4 {
            write.write_char('=');
        }
    }
}

fn construit_resultat(debug: bool, v2: &Vec<bool>, write: &mut impl WriteChar) -> (bool, i32, Option<Vec<bool>>) {
    let mut termine = false;
    let mut nb_affiche: i32 = 0;
    let mut res: Option<Vec<bool>> = None;
    if v2.len() >= 6 {
        let (debut, fin) = split(&v2, 6);

        if debug {
            println!("tmp={:?}", debut);
            println!("tmp2={:?}", fin);
        }

        let n = to_number(debut);

        if debug {
            println!("n={}({:b})", n, n);
        }

        affiche(n, write);
        nb_affiche += 1;

        res = Some(fin);
    } else if v2.len() == 0 {
        termine = true;
    } else {
        let mut v3: Vec<bool> = v2.clone();

        if debug {
            println!("v3={:?}", v3);
        }

        while v3.len() < 6 {
            v3.push(false);
        }

        if debug {
            println!("v3_bis={:?}", v3);
        }

        let n = to_number(v3);

        if debug {
            println!("n={}({:b})", n, n);
        }

        affiche(n, write);
        nb_affiche += 1;

        termine = true;
    }
    return (termine, nb_affiche, res);
}

fn main() {
    let mut input: Option<&str> = None;
    let mut output: Option<&str> = None;
    let args: Vec<String> = env::args().collect();
    let mut s: String = String::from("");
    let mut s2: String = String::from("");

    for arg in args {
        if arg.starts_with("--input=") {
            arg[8..].clone_into(&mut s);
            input = Some(&s);
        } else if arg.starts_with("--output=") {
            arg[9..].clone_into(&mut s2);
            output = Some(&s2);
        }
    }

    match input {
        Some(x) => {
            let my_buf = BufReader::new(File::open(x).unwrap());
            match output {
                Some(y) => {
                    let f = File::create(y).unwrap();
                    let mut out = BufWriter::new(f);
                    base64(my_buf, &mut out);
                }
                _ => {
                    let mut stdout = io::stdout();
                    base64(my_buf, &mut stdout);
                }
            }
        }
        _ => {
            let my_buf = BufReader::new(io::stdin());
            match output {
                Some(y) => {
                    let f = File::create(y).unwrap();
                    let mut out = BufWriter::new(f);
                    base64(my_buf, &mut out);
                }
                _ => {
                    let mut stdout = io::stdout();
                    base64(my_buf, &mut stdout);
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
    fn test_base64() {
        // test 'a'
        assert_eq!(base64t("a".as_bytes()), "YQ==");

        // test 'b'
        assert_eq!(base64t("b".as_bytes()), "Yg==");

        // test 'aa'
        assert_eq!(base64t("aa".as_bytes()), "YWE=");

        // test 'aaa'
        assert_eq!(base64t("aaa".as_bytes()), "YWFh");

        // test 'aaaa'
        assert_eq!(base64t("aaaa".as_bytes()), "YWFhYQ=="); //


        assert_eq!(base64t("light work.".as_bytes()), "bGlnaHQgd29yay4=");

        assert_eq!(base64t("light work".as_bytes()), "bGlnaHQgd29yaw==");

        assert_eq!(base64t("light wor".as_bytes()), "bGlnaHQgd29y");

        assert_eq!(base64t("light wo".as_bytes()), "bGlnaHQgd28=");

        assert_eq!(base64t("light w".as_bytes()), "bGlnaHQgdw==");
    }

    fn base64t(my_buf: impl io::BufRead) -> String {
        let mut s = String::new();
        base64(my_buf, &mut s);
        return s;
    }

    #[test]
    fn test_split() {
        assert_eq!(split(&vec![true, true, false, true], 2), (vec![true, true], vec![false, true]));

        assert_eq!(split(&vec![true, true, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, false, true], 6),
                   (vec![true, true, false, false, false, false],
                    vec![true, true, true, false, false, false, false, true, true, true, false, false, false, false, true]));
    }

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
    }

    fn base64t_bis(my_buf: impl io::BufRead) -> String {
        let mut s = String::new();
        base64_bis(my_buf, &mut s);
        s
    }
}
