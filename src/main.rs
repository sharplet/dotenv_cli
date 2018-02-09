extern crate dotenv;
extern crate exec;

macro_rules! die {
    ($msg:expr) => ({
        eprintln!("fatal: {}", $msg);
        std::process::exit(1);
    })
}

fn main() {
    dotenv::dotenv().ok();

    let mut args = std::env::args_os().skip(1);
    let name = args.next().unwrap_or_else(|| die!("no arguments"));
    let mut command = exec::Command::new(name);
    for argument in args {
        command.arg(argument);
    }
    let error = command.exec();
    die!(error);
}
