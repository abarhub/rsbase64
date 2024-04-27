use std::fs::File;
use std::io;
use std::io::BufReader;

static ARRAY: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

fn get_values(byte: u8) -> char {
    assert!(byte < 63);

    return ARRAY[byte as usize];
}

fn affiche(byte: u8, result: &mut Vec<char>) {
    assert!(byte < 63, "byte={}", byte);

    let c: char = get_values(byte);

    result.push(c);
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

fn base64(my_buf: impl io::BufRead) -> Vec<char> {
    let debug = false;
    let mut result2 = vec![];
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
            (_, nb_affiche2, res) = construit_resultat(debug, &mut result2, &v2);
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
        (termine, nb_affiche2, res) = construit_resultat(debug, &mut result2, &v2);
        match res {
            Some(v) => v2 = v,
            _ => {}
        }
        nb_affiche += nb_affiche2;
        no += 1;
    }
    if nb_affiche % 4 != 0 {
        for _ in (nb_affiche % 4)..4 {
            result2.push('=');
        }
    }

    return result2;
}

fn construit_resultat(debug: bool, mut result2: &mut Vec<char>, v2: &Vec<bool>) -> (bool, i32, Option<Vec<bool>>) {
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

        affiche(n, &mut result2);
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

        affiche(n, &mut result2);
        nb_affiche += 1;

        termine = true;
    }
    return (termine, nb_affiche, res);
}

fn main() {
    let my_buf = BufReader::new(File::open("./data/test2.txt").unwrap());

    let result = base64(my_buf);

    for c in result {
        print!("{}", c);
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base64() {
        // test 'a'
        assert_eq!(base64("a".as_bytes()), vec!['Y', 'Q', '=', '=']);

        // test 'b'
        assert_eq!(base64("b".as_bytes()), vec!['Y', 'g', '=', '=']);

        // test 'aa'
        assert_eq!(base64("aa".as_bytes()), vec!['Y', 'W', 'E', '=']);

        // test 'aaa'
        assert_eq!(base64("aaa".as_bytes()), vec!['Y', 'W', 'F', 'h']);

        // test 'aaaa'
        assert_eq!(base64("aaaa".as_bytes()), vec!['Y', 'W', 'F', 'h', 'Y', 'Q', '=', '=']);//


        assert_eq!(base64("light work.".as_bytes()), vec!['b', 'G', 'l', 'n', 'a', 'H', 'Q', 'g', 'd', '2', '9', 'y', 'a', 'y', '4', '=']);//bGlnaHQgd29yay4=

        assert_eq!(base64("light work".as_bytes()), vec!['b', 'G', 'l', 'n', 'a', 'H', 'Q', 'g', 'd', '2', '9', 'y', 'a', 'w', '=', '=']);

        assert_eq!(base64("light wor".as_bytes()), vec!['b', 'G', 'l', 'n', 'a', 'H', 'Q', 'g', 'd', '2', '9', 'y']);

        assert_eq!(base64("light wo".as_bytes()), vec!['b', 'G', 'l', 'n', 'a', 'H', 'Q', 'g', 'd', '2', '8', '=']);

        assert_eq!(base64("light w".as_bytes()), vec!['b', 'G', 'l', 'n', 'a', 'H', 'Q', 'g', 'd', 'w', '=', '=']);
    }


    #[test]
    fn test_split() {
        assert_eq!(split(&vec![true, true, false, true], 2), (vec![true, true], vec![false, true]));

        assert_eq!(split(&vec![true, true, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, false, true], 6),
                   (vec![true, true, false, false, false, false],
                    vec![true, true, true, false, false, false, false, true, true, true, false, false, false, false, true]));
    }
}
