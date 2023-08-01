mod cesar;
mod parser;
mod base_cryptography;

use linefeed::{Interface, ReadResult};
use base_cryptography::Algo;
use parser::{parse, commands::Command};

//type Reader = Interface<linefeed::DefaultTerminal> ;

fn select_algo(algo: &str, context: &Algo) -> (Algo, String) {
    match algo {
        "cesar" => (Algo::new("cesar"), "Cesar algorithm selected.".to_string()),
        _ => (context.clone(), " ".to_string())
    }
}

fn help_message(_rest: &str, context: &Algo) -> (Algo, String) {
    let help = format!("select: select an algo\nencrypt\t key message status exit help");
   (context.clone(), help) 
}

fn eval(input: &str, context: &Algo) -> (Algo, String) {
   match parse(input) {
       Command::Exit => (Algo::Exit, "exit".to_string()),
       Command::Select(algo) => select_algo(&algo, context),
       Command::Encrypt(None) => (context.clone(), context.encrypt2()),
       Command::Encrypt(Some((k, m))) => (context.clone(), context.encrypt(&m, k)),
       Command::Decrypt(None) => (context.clone(), context.decrypt2()),
       Command::Decrypt(Some((k, m))) => (context.clone(), context.decrypt(&m, k)),
       Command::Key(k) => (context.set_key(k), "Key value set".to_string()),
       Command::Message(m) => (context.set_message(&m), "Message set".to_string()),
       Command::Status => (context.clone(), context.get_status()),
       Command::Help => help_message("", context),
       _ => (context.clone(), "".to_string())
   }
}

fn initialize() -> (Interface<linefeed::DefaultTerminal>, Algo){
    let reader = Interface::new("Cryptography").unwrap();
    reader.set_prompt(":c:> ").unwrap();
    (reader, Algo::Empty)
}


fn main() {
    let (reader, mut context) = initialize();
    let mut res = "".to_string();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        (context, res) = eval(&input, &context);
        println!("{:?}", res);
        if context == Algo::Exit {
            std::process::exit(0);
        } else {
            reader.set_prompt(&context.get_prompt()).unwrap();
        }
    }
    println!("Goodbye.");
}

