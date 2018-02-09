extern crate clap;
extern crate dotenv;
extern crate exec;

use clap::App;
use clap::AppSettings;
use clap::Arg;
use exec::Command;

macro_rules! die {
    ($msg:expr) => ({
        eprintln!("fatal: {}", $msg);
        std::process::exit(1);
    })
}

fn main() {
    let matches = App::new("dotenv")
        .about("Run a command using the environment in a .env file")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .takes_value(true)
             .help("Use a specific .env file (defaults to .env)"))
        .arg(Arg::with_name("command")
             .required(true))
        .arg(Arg::with_name("args..."))
        .get_matches();

    match matches.value_of("file") {
        None => dotenv::dotenv(),
        Some(file) => dotenv::from_filename(file),
    }.ok();

    let mut command = Command::new(matches.value_of("command").unwrap());

    match matches.values_of("args") {
        Some(command_args) => {
            for argument in command_args {
                command.arg(argument);
            }
        },
        None => {},
    };

    let error = command.exec();
    die!(error);
}
