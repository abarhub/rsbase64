use std::fs::File;
use std::io;
use std::io::BufReader;

// let debug=false;

fn get2(byte: u8, pos: u8) -> (u8, u8) {
    assert!(pos < 7);

    let pos2 = 7 - pos;

    let debut = get(byte, 0, pos2 - 1);
    let fin = get(byte, pos2, 7);

    //println!("get2 {:b} -> {:b}, {:b}",byte, debut,fin);

    //return (debut,fin);
    return (fin, debut);
}

/**
Return the bits of byte between pos_debut and pos_fin.
The position is between the less significant bits (right to left)

get(0b1010101,2,7)=0b0010101
get(0b1010101,0,2)=0b0000001

 */
fn get(byte: u8, pos_debut: u8, pos_fin: u8) -> u8 {
    assert!(pos_debut < pos_fin);

    let mut res: u8 = byte;
    if pos_fin < 7 {
        res <<= 7 - pos_fin;
        res >>= 7 - pos_fin;
    }
    if pos_debut > 0 {
        res >>= pos_debut;
    }

    return res;
}

fn get_values(byte: u8) -> char {
    assert!(byte < 63);

    let array: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

    return array[byte as usize];
}

fn affiche(byte: u8, result: &mut Vec<char>) {
    assert!(byte < 63, "byte={}", byte);

    // let array: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    //     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    //     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

    // let s2: String = array.iter().collect();

    // afficher le caractère
    //println!("{}",array[byte as u32]);
    //println!("{}",s2[2]);

    let c: char = get_values(byte);

    //println!("{}", c);
    // print!("{}", c);
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

fn split(vect: Vec<bool>, pos: i8) -> (Vec<bool>, Vec<bool>) {
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
    // let debug = true;
    let debug = false;
    // let mut res: u8 = 0;
    // let mut len_res = 0;
    // let mut no = 0;
    // let mut result = vec![];
    let mut result2 = vec![];
    let mut v2 = vec![];
    // let mut len2 = 0;
    for byte_or_error in my_buf.bytes() {
        // assert!(len_res <= 8);
        // assert_eq!(len_res % 2, 0);

        // let n:u8 = 0;

        let byte = byte_or_error.unwrap();
        // let debut = byte & 0b0011_1111;
        // let fin = (byte & 0b1100_0000) >> 6;

        // if(len_res==0){
        //     let debut=get(byte,0,5-len_res);
        //     let fin=get(byte,6-len_res,7);
        //
        //     let n = res+(debut<<len_res);
        //
        //     // traitement
        //     affiche(n);
        //
        //     res=fin;
        //     len_res=(len_res+2)%6;
        // } else if(len_res==2){
        //     let debut=get(byte,0,5-len_res);
        //     let fin=get(byte,6-len_res,7);
        //
        //     let n = res+(debut<<len_res);
        //
        //     affiche(n);
        //
        //     res=fin;
        //     len_res=(len_res+2)%6;
        //
        // } else if(len_res==4){
        // let debut = get(byte, 0, 5 - len_res);
        // let fin = get(byte, 6 - len_res, 7);

        let v = create_vector(byte);

        v2 = [v2, v].concat();

        // println!("debut v={:?}, byte={}({:b})",
        //          v, byte, byte);

        // if debug {
        //     println!("debut no={},res={}({:b}),len={}",
        //              no, res, res, len_res);
        // }

        //let (debut2,fin2)=split(v,7-len2);

        //let (debut2,fin2)=split(v,6);


        // let (debut, fin) = get2(byte, 5 - len_res);
        //
        // let n;//= res<<(7-len_res) + debut;
        // if res > 0 {
        //     n = (res << (6 - len_res)) + debut;
        // } else {
        //     n = debut;
        // }
        //
        // if debug {
        //     println!("no={},byte={}({:b}),debut={}({:b}),fin={}({:b}),n={},res={},len={},n={}({:b})",
        //              no, byte, byte, debut, debut, fin, fin, n, res, len_res, n, n);
        // }
        //
        // affiche(n, &mut result);
        //
        // // if len_res + 2 == 6 {
        // //     affiche(fin, &mut result);
        // //
        // //     res = 0;
        // //     len_res = 0;
        // // } else {
        // res = fin;
        // len_res = (len_res + 2) % 6;
        // // }
        //
        // if debug {
        //     println!("no_bis={},res={}({:b})len={}", no, res, res, len_res);
        // }
        //
        // no += 1;
        //
        // assert!(len_res <= 8);
        // assert_eq!(len_res % 2, 0);
    }

    // if debug {
    //     println!("fin boucle res={}({:b})len={}", res, res, len_res);
    // }

    // if len_res > 0 {
    //     let res2;
    //     if len_res < 6 {
    //         res2 = res << (6 - len_res);
    //     } else {
    //         res2 = res;
    //     }
    //
    //     if debug {
    //         println!("res2={}({:b})", res2, res2);
    //     }
    //
    //     // let (debut, fin) = get2(res2, 5 - len_res);
    //
    //     // println!("debut={}({:b}),fin={}({:b})", debut, debut, fin, fin);
    //
    //     let n = res2;
    //
    //     affiche(n, &mut result);
    //
    //     if len_res == 2 {
    //         result.push('=');
    //         result.push('=');
    //         // println!("==");
    //     } else if len_res == 1 || len_res == 4 {
    //         // println!("=");
    //         result.push('=');
    //     }
    //
    //     // if len_res + 2 == 6 {
    //     //     affiche(fin);
    //     //
    //     //     res = 0;
    //     //     len_res = 0;
    //     // } else {
    //     //     res = fin;
    //     //     len_res = (len_res + 2) % 6;
    //     // }
    // } else if res > 0 {
    //     let n = res;
    //
    //     affiche(n, &mut result);
    // }

    // while result.len()%3!=0 {
    //     result.push('=');
    // }

    if debug {
        println!("construction du resultat: v2={:?}", v2);
    }

    let mut termine = false;
    let mut no = 0;
    let mut nb_affiche = 0;
    while !termine {
        if debug {
            println!("no={}", no);
            println!("v2(len={})={:?}", v2.len(), v2);
        }
        if v2.len() >= 6 {
            let (debut, fin) = split(v2, 6);

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

            v2 = fin;
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
        no += 1;
    }
    if nb_affiche % 4 != 0 {
        for _ in (nb_affiche % 4)..4 {
            result2.push('=');
        }
    }

    // return result;
    return result2;
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
    fn test_get() {
        // test ' '
        assert_eq!(get(32, 0, 5), 32);
        assert_eq!(get(32, 6, 7), 0);

        // test 'a'
        assert_eq!(get(97, 0, 5), 33);
        assert_eq!(get(97, 6, 7), 1);

        assert_eq!(get(97, 0, 1), 1);
        assert_eq!(get(97, 2, 7), 24);
    }


    #[test]
    fn test_get2() {
        // test ' '
        assert_eq!(get2(32, 5), (8, 0));

        // test 'a'
        assert_eq!(get2(97, 5), (24, 1));

        // test 197
        assert_eq!(get2(197u8, 5), (49, 1));
    }

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
        assert_eq!(split(vec![true, true, false, true], 2), (vec![true, true], vec![false, true]));

        assert_eq!(split(vec![true, true, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, false, true], 6),
                   (vec![true, true, false, false, false, false],
                    vec![true, true, true, false, false, false, false, true, true, true, false, false, false, false, true]));
    }
}
