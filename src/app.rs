
use clap::{Arg, App, SubCommand, ArgMatches};
use super::validate::{validate};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};
use serde_json;
use super::dit_core::message::Message;

pub fn get_app<'a, 'b>() -> App<'a, 'b> {
 return App::new("dit")
        .version("0.1")
        .author("Logan W, testare.i9z@gmail.com")
        .about("A CLI game")
        .arg(Arg::with_name("config").short("c").long("config").value_name("CONFIG").help("Sets a config file").takes_value(true))
        .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
        .subcommand(subcommand_raw_add())
        .subcommand(subcommand_validate());


}

pub fn handle_matches(app_m: ArgMatches) {
    let config_arg = app_m.value_of("config");
    match app_m.subcommand() {
        ("test", Some(arg_m)) => {
            println!("{:?} {:?}", config_arg, arg_m.is_present("debug"));
        }
        ("rawadd", Some(arg_m)) => {
            let file_name: &str = arg_m.value_of("filename").unwrap_or(".dit");

            let last_message: Message;

            if let Ok(file) = File::open(file_name) {
                let lines = BufReader::new(file).lines();
                last_message = lines
                    .last()
                    .map (|message_str| {
                        println!("In the opt map with {:?}", message_str);
                        serde_json::from_str::<Message>(message_str.unwrap().as_str()).unwrap() // Fix unwrapping later
                    }).unwrap();
            } else {
                // println!("There was a problem loading that file [{}]", file_name);
                if let Ok(_) = File::create(file_name) {
                    last_message = Message::default()
                } else {
                    panic!("Unable to find/create file {}", &file_name);
                }
            }

            let message_payload: &str = arg_m.value_of("content").unwrap();
            let next_message = last_message.gen_next_message(message_payload, 5);
            let mut file = OpenOptions::new().append(true).open(file_name).unwrap();
            let string: String = serde_json::to_string(&next_message).unwrap();
            write!(file, "{}\n", string).unwrap();
        }
        ("validate", Some(arg_m)) => {
            let file_name: &str = arg_m.value_of("filename").unwrap_or("dummy");
            let validation_result = validate(file_name);
            match validation_result {
                Ok(_) => println!("I would consider {} as valid", file_name),
                Err(err) => println!("{}", err),
            }
        }
        _ => {
            println!("TODO We should probably print available subcommands in this case. Perhaps with help of the man crate?");
        }
    }
}

//| Subcommand to add a raw message to a file
fn subcommand_raw_add<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("rawadd")
        .arg(Arg::with_name("filename").short("f").long("filename").value_name("FILENAME").help("Sets the file to add message").takes_value(true))
        .arg(Arg::with_name("content").help("Select file to validate").required(true).index(1));
}

//| Subcommand to validate a file.
fn subcommand_validate<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("validate")
        .arg(Arg::with_name("filename").help("Select file to validate").required(true).index(1));
}