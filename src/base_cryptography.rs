#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

use crate::cesar::Cesar;
use crate::vigenere::Vigenere;
use crate::parser::commands::KeyValue;

pub trait Key: std::ops::Add + Sized { }

pub trait Cipher { }

#[derive(PartialEq, Debug, Clone)]
pub enum Algo {
    Cesar(Cesar),
    Vigenere(Vigenere),
    Empty,
    Exit,
}

impl Algo {
    pub fn from(algo: Algo) -> Algo {
        match algo {
            Algo::Cesar(_) => Algo::Cesar(Cesar {key: 0, message: "".to_string()}),
            _ => Algo::Empty
        }
    }

    pub fn new(algo: &str) -> Algo {
        match algo {
            "cesar" => Algo::Cesar(Cesar {key: 0, message: "".to_string()}),
            "vigenere" => Algo::Vigenere(Vigenere {key: "".to_string(), message: "".to_string()}),
            _ => Algo::Empty
        }
    }

    pub fn encrypt(&self, msg: &str, key: KeyValue) -> String {
       match (self, key.clone()) {
           (Algo::Cesar(c), KeyValue::Integer(i)) => c.encrypt(msg, i),
           (Algo::Vigenere(v), KeyValue::String(s)) => v.encrypt(msg, &s),
           _ => format!("The algorithm {:?} don't work with a key {}. hint: try to change the key type", self, &key.describe())
       } 
    }

    pub fn encrypt2(&self) -> String {
       match self {
           Algo::Cesar(c) => c.encrypt2(),
           Algo::Vigenere(v) => v.encrypt2(),
           _ => "The algorithm doesn't exist".to_string()
       } 
    }

    pub fn decrypt(&self, msg: &str, key: KeyValue) -> String {
       match (self, key.clone()) {
           (Algo::Cesar(c), KeyValue::Integer(i)) => c.decrypt(msg, i),
           (Algo::Vigenere(v), KeyValue::String(s)) => v.decrypt(msg, &s),
           _ => format!("The algorithm {:?} don't work with a key {}. hint: try to change the key type", self, key.describe())
       } 
    }

    pub fn decrypt2(&self) -> String {
       match self {
           Algo::Cesar(c) => c.decrypt2(),
           Algo::Vigenere(v) => v.decrypt2(),
           _ => "The algorithm doesn't exist".to_string()
       } 
    }

    pub fn set_key(&self, key: KeyValue) -> Algo {
        match (self, key.clone()) {
            (Algo::Cesar(c), KeyValue::Integer(i)) => Algo::Cesar(c.set_key(i)),
            (Algo::Vigenere(v), KeyValue::String(s)) => Algo::Vigenere(v.set_key(&s)),
            (Algo::Empty, _) => Algo::Empty,
            (Algo::Exit, _) => Algo::Exit,
            (Algo::Cesar(c), _) => Algo::Cesar(c.clone()),
            (Algo::Vigenere(v), _) => Algo::Vigenere(v.clone()),
        }
    }

    pub fn get_key(&self) -> i8 {
        match self {
            Algo::Cesar(c) => c.get_key(),
            _ => 0
        }
    }

    pub fn get_prompt(&self) -> String {
       match self {
           Algo::Cesar(_) => ":[Cesar]:> ".to_string(),
           Algo::Vigenere(_) => ":[Vigenere]:> ".to_string(),
           _ => ":[?]:> ".to_string()
       } 
    }

    pub fn set_message(&self, message: &str) -> Algo {
        match self {
            Algo::Cesar(c) => Algo::Cesar(c.set_message(message)),
            _ => Algo::Empty
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Algo::Cesar(c) => c.get_message(),
            _ => "".to_string()
        }
    }

    pub fn get_algo(&self) -> String {
        match self {
            Algo::Cesar(_) => "cesar".to_string(),
            Algo::Vigenere(_) => "vigenere".to_string(),
            Algo::Empty => "[not defined]".to_string(),
            Algo::Exit => "[not defined]".to_string()
        }
    }

    pub fn get_status(&self) -> String {
        match self {
            algo => format!("algo: {} | key: {} | message: {}", algo.get_algo(), algo.get_key(), algo.get_message())
        }
    }

    pub fn list_algo() -> String {
        "??".to_string()
    }

}
