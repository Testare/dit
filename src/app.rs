use super::dit_core::{validate, with_game_state, read_state, ActionInterface};
use super::mode_a::ActionA;
use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("dit")
        .version("0.1")
        .author("Logan W, testare.i9z@gmail.com")
        .about("A CLI game")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG")
                .help("Sets a config file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .subcommand(subcommand_raw_add())
        .subcommand(subcommand_update())
        .subcommand(subcommand_validate())
}

// Should later change it to have its own Writer
pub fn handle_matches(app_m: ArgMatches) {
    let config_arg = app_m.value_of("config");
    match app_m.subcommand() {
        ("update", Some(arg_m)) => {
            let file_name: &str = arg_m.value_of("filename").unwrap_or(".dit");
            let version: usize = arg_m.value_of("version").unwrap_or("5").parse::<>().unwrap();
            let (state, ledger) = read_state::<ActionA>(file_name).unwrap();

            ActionInterface::new()
                .with_period(1)
                .on_iter(|hex_string| println!("-> {}", hex_string.to_string()))
                .on_fail(|| println!("Oooooh, we failed"))
                .on_success(|hex_string| println!("-> {} wins!", hex_string.to_string()))
                .run(ActionA::UpdateVersion{version}, ledger, state)
                .unwrap();
        }
        ("test", Some(arg_m)) => {
            println!("{:?} {:?}", config_arg, arg_m.is_present("debug"));
        }
        ("rawadd", Some(arg_m)) => {
            let file_name: &str = arg_m.value_of("filename").unwrap_or(".dit");
            let message_payload: &str = arg_m.value_of("content").unwrap();
            let k = with_game_state(file_name, |_| {
                Ok(ActionA::Marker {
                    content: String::from(message_payload),
                })
            });
            k.unwrap();
        }
        ("validate", Some(arg_m)) => {
            let file_name: &str = arg_m.value_of("filename").unwrap_or("dummy");
            let validation_result = validate::<ActionA>(file_name);
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
        .arg(
            Arg::with_name("filename")
                .help("Sets the file to add message")
                .long("filename")
                .short("f")
                .takes_value(true)
                .value_name("FILENAME")
        )
        .arg(
            Arg::with_name("content")
                .help("Select file to validate")
                .required(true)
                .index(1),
        );
}

//| Subcommand to validate a file.
fn subcommand_validate<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("validate").arg(
        Arg::with_name("filename")
            .help("Select file to validate")
            .index(1)
            .required(true),
    );
}

fn subcommand_update<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update").arg(
        Arg::with_name("filename")
            .help("Select file to update")
            .index(1)
            .required(false)
    )
    .arg(
        Arg::with_name("version")
            .help("The version target for the update")
            .long("version")
            .short("v")
            .takes_value(true)
            .value_name("VERSION")
    )

}