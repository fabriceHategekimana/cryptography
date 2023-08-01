#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

use crate::base_cryptography::{Key, Cipher};
//use std::ops::Add;

#[derive(PartialEq, Debug, Clone)]
pub struct Cesar {
    pub key: i8,
    pub message: String
}

impl Key for i8 { }

impl Cipher for String { }

fn get_alphabet() -> Vec<(char, i8)> {
    vec![ ('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4),
        ('f', 5), ('g', 6), ('h', 7), ('i', 8), ('j', 9),
        ('k', 10), ('l', 11), ('m', 12), ('n', 13), ('o', 14),
        ('p', 15), ('q', 16), ('r', 17), ('s', 18), ('t', 19),
        ('u', 20), ('v', 21), ('w', 22), ('x', 23), ('y', 24),
        ('z', 25) ]
}

fn letter_to_number(c: char) -> i8 {
    let alphabet =  get_alphabet();
    let mut res = alphabet.iter().filter(|x| x.0 == c);
    if let Some(couple) = res.next() {
        couple.1
    } else {
        -1
    }
}

fn compute<N>(key: i8) -> impl Fn(i8) -> i8 {
   move |x| (x + key) % 26
}

fn number_to_letter(n: i8) -> char {
    let alphabet = get_alphabet();
    let mut res = alphabet.iter().filter(|x| x.1 == n);
    if let Some(couple) = res.next() {
        couple.0
    } else {
        ' '
    }
}

impl Cesar {
    pub fn encrypt(&self, msg: &str, key: i8) -> String {
        msg.chars()
            .map(letter_to_number)
            .map(compute::<i8>(key))
            .map(number_to_letter)
            .collect::<String>()
    }

    pub fn encrypt2(&self) -> String {
        self.message.chars()
            .map(letter_to_number)
            .map(compute::<i8>(self.key))
            .map(number_to_letter)
            .collect::<String>()
    }

    pub fn decrypt(&self, ct: &str, key: i8) -> String {
        self.encrypt(ct, -key)
    }

    pub fn decrypt2(&self) -> String {
        self.encrypt(&self.message, -self.key)
    }

    pub fn set_key(&self, value: i8) -> Cesar {
        Cesar {
            key: value,
            message: self.message.clone()
        }
    }

    pub fn get_key(&self) -> i8 {
        self.key
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn set_message(&self, message: &str) -> Cesar {
        Cesar {
            key: self.key,
            message: message.to_string()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        assert_eq!( letter_to_number('a'), 0);
        assert_eq!( letter_to_number('z'), 25);
    }

    #[test]
    fn test_number_to_letter() {
        assert_eq!(
            number_to_letter(0),
            'a');
    }

    #[test]
    fn test_encrypt() {
        assert_eq!(
            Cesar{ key: 2, message: "a".to_string() }.encrypt2(),
            "c"); 
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            Cesar{ key: 2, message: "c".to_string() }.decrypt2(),
            "a"); 
    }

}
