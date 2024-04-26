use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;


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

fn affiche(byte: u8, mut result: &mut Vec<char>) {
    assert!(byte < 63);

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

fn base64(//my_buf: BufReader<File>
          my_buf: impl io::BufRead
) -> Vec<char> {
    let mut res: u8 = 0;
    let mut len_res = 0;
    let mut no = 0;
    let mut result = vec![];
    for byte_or_error in my_buf.bytes() {
        assert!(len_res <= 8);
        assert_eq!(len_res % 2, 0);

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

        let (debut, fin) = get2(byte, 5 - len_res);

        let n = res + (debut << len_res);

        //println!("no={},byte={}({:b}),debut={}({:b}),fin={}({:b}),n={},res={},len={}",
        //         no, byte, byte, debut, debut, fin, fin, n, res, len_res);

        affiche(n, &mut result);

        if len_res + 2 == 6 {
            affiche(fin, &mut result);

            res = 0;
            len_res = 0;
        } else {
            res = fin;
            len_res = (len_res + 2) % 6;
        }

        //println!("no_bis={},res={}({:b})len={}", no, res, res, len_res);

        no += 1;

        assert!(len_res <= 8);
        assert_eq!(len_res % 2, 0);
    }

    //println!("fin boucle res={}({:b})len={}", res, res, len_res);

    if len_res > 0 {
        let res2;
        if (len_res < 6) {
            res2 = res << (6 - len_res);
        } else {
            res2 = res;
        }

        //println!("res2={}({:b})", res2, res2);

        let (debut, fin) = get2(res2, 5 - len_res);

        //println!("debut={}({:b}),fin={}({:b})", debut, debut, fin, fin);

        let n = res2;

        affiche(n, &mut result);

        if (len_res == 2) {
            result.push('=');
            result.push('=');
            // println!("==");
        } else if (len_res == 1) {
            // println!("=");
            result.push('=');
        }

        // if len_res + 2 == 6 {
        //     affiche(fin);
        //
        //     res = 0;
        //     len_res = 0;
        // } else {
        //     res = fin;
        //     len_res = (len_res + 2) % 6;
        // }
    }

    return result;
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
        // test ' '
        assert_eq!(base64("a".as_bytes()), vec!['Y', 'Q', '=', '=']);
    }
}
