use clap::{App, Arg, SubCommand};

fn unimpl() {
    eprintln!("unimplemented");
    std::process::exit(1);
}

fn main() {
        let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Qi <295872776@qq.com>")
        .about("A key-value store")
        .subcommand(
            SubCommand::with_name("set")
                .args(&[
                    Arg::with_name("key").takes_value(true),
                    Arg::with_name("value").takes_value(true),
                ])
                .help("Set the value of a string key to a string"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(Arg::with_name("key").takes_value(true))
                .help("Get the string value of a given string key"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("set") => unimpl(),
        Some("get") => unimpl(),
        None => std::process::exit(1),
        _ => unreachable!(),
    }
}