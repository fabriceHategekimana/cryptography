#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

use crate::base_cryptography::{Key, Cipher};
use crate::tools::{get_alphabet, letter_to_number, number_to_letter};

#[derive(PartialEq, Debug, Clone)]
pub struct Cesar {
    pub key: i8,
    pub message: String
}

impl Key for i8 { }

impl Cipher for String { }

fn compute<N>(key: i8) -> impl Fn(i8) -> i8 {
   move |x| (x + key) % 26
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
