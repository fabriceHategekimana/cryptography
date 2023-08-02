// tools for cryptography

pub fn get_alphabet() -> Vec<(char, i8)> {
    vec![ ('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4),
        ('f', 5), ('g', 6), ('h', 7), ('i', 8), ('j', 9),
        ('k', 10), ('l', 11), ('m', 12), ('n', 13), ('o', 14),
        ('p', 15), ('q', 16), ('r', 17), ('s', 18), ('t', 19),
        ('u', 20), ('v', 21), ('w', 22), ('x', 23), ('y', 24),
        ('z', 25) ]
}

pub fn letter_to_number(c: char) -> i8 {
    let alphabet =  get_alphabet();
    let mut res = alphabet.iter().filter(|x| x.0 == c);
    if let Some(couple) = res.next() {
        couple.1
    } else {
        -1
    }
}

pub fn number_to_letter(n: i8) -> char {
    let alphabet = get_alphabet();
    let mut res = alphabet.iter().filter(|x| x.1 == n);
    if let Some(couple) = res.next() {
        couple.0
    } else {
        ' '
    }
}
