#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

//parseur de commande
pub mod commands;

use nom::IResult;
use nom::sequence::preceded;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::multi::many1;
use nom::character::complete::space1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::character::complete::digit1;
use nom::sequence::terminated;
use nom::branch::alt;
use commands::Command;
use crate::KeyValue;

fn parse_select(s: &str) -> IResult<&str,Command> {
    let res = preceded(
        tag("select "),
        alphanumeric1
          )(s);
    match res {
        Ok((s, name)) => Ok((s, Command::Select(name.to_string()))),
        Err(r) => Err(r)
    }
}

fn parse_words_and_spaces(s: &str) -> IResult<&str, String> {
    let res = many1(alt((
        terminated(alphanumeric1, space1),
        alphanumeric1
        )))(s);
    match res {
        Ok((s, v)) => Ok((s, v.join(" "))),
        Err(r) => Err(r)
    }
}

fn parse_text(s: &str) -> IResult<&str,String> {
    delimited(
        tag("\""),
        parse_words_and_spaces,
        tag("\""))(s)
}

fn parse_encrypt_full(s: &str) -> IResult<&str,Command> {
    let res = tuple((
        tag("encrypt "),
        parse_text,
        tag(" with "),
        digit1,
        ))(s);
    match res {
        Ok((s, (e, m, w, d))) => Ok((s, Command::Encrypt(Some((
                            KeyValue::Integer(parse_digit(d)),
                            m))))),
        Err(e) => Err(e)
    }
}

fn parse_encrypt_simple(s: &str) -> IResult<&str,Command> {
    let res = tag("encrypt")(s);
    match res {
        Ok((s, e)) => Ok((s, Command::Encrypt(None))),
        Err(r) => Err(r)
    }
}

fn parse_encrypt(s: &str) -> IResult<&str,Command> {
    alt((
            parse_encrypt_full,
            parse_encrypt_simple
        ))(s)
}

fn parse_decrypt_full(s: &str) -> IResult<&str,Command> {
    let res = tuple((
        tag("decrypt "),
        parse_text,
        tag(" with "),
        digit1,
        ))(s);
    match res {
        Ok((s, (e, m, w, d))) => Ok((s, Command::Decrypt(Some((
                            KeyValue::Integer(parse_digit(d)),
                            m))))),
        Err(e) => Err(e)
    }
}

fn parse_decrypt_simple(s: &str) -> IResult<&str,Command> {
    let res = tag("decrypt")(s);
    match res {
        Ok((s, e)) => Ok((s, Command::Decrypt(None))),
        Err(r) => Err(r)
    }
}

fn parse_decrypt(s: &str) -> IResult<&str,Command> {
    alt((
            parse_decrypt_full,
            parse_decrypt_simple
        ))(s)
}

fn parse_digit(s: &str) -> i8 {
    match s.parse::<i8>() {
        Ok(v) => v,
        _ => 0_i8
    }
}

fn parse_key(s: &str) -> IResult<&str,Command> {
    let res = preceded(
                tag("key "),
                digit1)(s);
    match res {
        Ok((s, d)) => Ok((s, Command::Key(parse_digit(d)))),
        Err(r) => Err(r)
    }
}

fn parse_message(s: &str) -> IResult<&str,Command> {
    let res = preceded(
            tag("message "),
            parse_text)(s);
    match res {
        Ok((s, t)) => Ok((s, Command::Message(t))),
        Err(r) => Err(r)
    }
}

fn parse_status(s: &str) -> IResult<&str,Command> {
    match tag("status")(s) {
        Ok((s, _)) => Ok((s, Command::Status)),
        Err(r) => Err(r)
    }
}

fn parse_exit(s: &str) -> IResult<&str,Command> {
    match tag("exit")(s) {
        Ok((s, _)) => Ok((s, Command::Exit)),
        Err(r) => Err(r)
    }
}

fn parse_help(s: &str) -> IResult<&str,Command> {
    match tag("help")(s) {
        Ok((s, _)) => Ok((s, Command::Help)),
        Err(r) => Err(r)
    }
}

pub fn parse(s: &str) -> Command {
    let res = alt((
        parse_select,
        parse_encrypt,
        parse_decrypt,
        parse_key,
        parse_message,
        parse_status,
        parse_exit,
        parse_help,
          ))(s);
    match res {
        Ok((s, cmd)) => cmd,
        Err(r) => Command::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select(){
        assert_eq!(
            parse_select("select cesar").unwrap().1,
            Command::Select("cesar".to_string()));
    }

    #[test]
    fn test_words_and_spaces() {
        assert_eq!(
            parse_words_and_spaces("hello world").unwrap().1,
            "hello world".to_string());
    }
    
    #[test]
    fn test_parse_key() {
        assert_eq!(
            parse_key("key 67").unwrap().1,
            Command::Key(67_i8));
    }

    #[test]
    fn test_encrypt() {
        assert_eq!(
            parse_encrypt("encrypt \"Hello\" with 5").unwrap().1,
            Command::Encrypt(Some((KeyValue::Integer(5), "Hello".to_string()))));
    }

    #[test]
    fn test_message() {
        assert_eq!(
            parse_message("message \"Hello\"").unwrap().1,
            Command::Message("Hello".to_string()));
    }

    #[test]
    fn test_status() {
        assert_eq!(
            parse_status("status").unwrap().1,
            Command::Status);
    }

    #[test]
    fn test_exit() {
        assert_eq!(
            parse_exit("exit").unwrap().1,
            Command::Exit);
    }

    #[test]
    fn test_help() {
        assert_eq!(
            parse_help("help").unwrap().1,
            Command::Help);
    }

}
