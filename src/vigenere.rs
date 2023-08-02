// Implementing the vigenaire algorythm

use crate::tools::{get_alphabet, letter_to_number, number_to_letter};

#[derive(PartialEq, Debug, Clone)]
pub struct Vigenere {
    pub key: String,
    pub message: String
}

fn inverse_key(key: &str) -> String {
    key.chars()
        .map(letter_to_number)
        .map(|x| (26-x) % 26)
        .map(number_to_letter)
        .collect::<String>()
}


impl Vigenere {
    pub fn encrypt(&self, msg: &str, key: &str) -> String {
       let keys = key.chars().map(letter_to_number).collect::<Vec<i8>>();
       let base = keys.len();
       msg.chars().map(letter_to_number)
           .enumerate()
           .map(|x| (keys[x.0 % base], x.1))
           .map(|x| (x.0 + x.1) % 26_i8)
           .map(number_to_letter).collect()
    }

    pub fn encrypt2(&self) -> String {
        self.encrypt(&self.message, &self.key)
    }

    pub fn decrypt(&self, msg: &str, key: &str) -> String {
        self.encrypt(msg, &inverse_key(key))
    }

    pub fn decrypt2(&self) -> String {
        self.decrypt(&self.message, &self.key)
    }

    pub fn set_key(&self, key: &str) -> Vigenere {
        Vigenere {
            key: key.to_string(),
            message: self.message.clone()
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_encrypt(){
        let v = Vigenere { message: "hello".to_string().to_string(), key: "b".to_string() };
        assert_eq!(
            v.encrypt2(),
            "ifmmp".to_string());
    }

    #[test]
    fn test_vigenere_encrypt2(){
        let v = Vigenere { message: "hello".to_string(), key: "bc".to_string() };
        assert_eq!(
            v.encrypt2(),
            "igmnp".to_string());
    }

    #[test]
    fn test_vigenere_decrypt(){
        let v = Vigenere { message: "ifmmp".to_string(), key: "b".to_string() };
        assert_eq!(
            v.decrypt2(),
            "hello".to_string());
    }

    #[test]
    fn test_inverse_key() {
        assert_eq!(
            inverse_key("a"),
            "a".to_string());
    }


}
