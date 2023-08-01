#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

use crate::cesar::Cesar;

pub trait Key: std::ops::Add + Sized { }

pub trait Cipher { }

#[derive(PartialEq, Debug, Clone)]
pub enum Algo {
    Cesar(Cesar),
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
            _ => Algo::Empty
        }
    }

    pub fn encrypt(&self, msg: &str, key: i8) -> String {
       match self {
           Algo::Cesar(c) => c.encrypt(msg, key),
           _ => "The algorithm doesn't exist".to_string()
       } 
    }

    pub fn encrypt2(&self) -> String {
       match self {
           Algo::Cesar(c) => c.encrypt2(),
           _ => "The algorithm doesn't exist".to_string()
       } 
    }

    pub fn decrypt(&self, msg: &str, key: i8) -> String {
       match self {
           Algo::Cesar(c) => c.decrypt(msg, key),
           _ => "The algorithm doesn't exist".to_string()
       } 
    }

    pub fn decrypt2(&self) -> String {
       match self {
           Algo::Cesar(c) => c.decrypt2(),
           _ => "The algorithm doesn't exist".to_string()
       } 
    }

    //TODO return a Algo each time
    pub fn set_key(&self, key: i8) -> Algo {
        match self {
            Algo::Cesar(c) => Algo::Cesar(c.set_key(key)),
            _ => Algo::Empty
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
            _ => "unknown".to_string()
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
