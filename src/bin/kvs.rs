use clap::{App, AppSettings, Arg, SubCommand};

fn unimpl() {
    eprintln!("unimplemented");
    std::process::exit(1);
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("set")
                .args(&[
                    Arg::with_name("key")
                        .help("A string key")
                        .takes_value(true)
                        .required(true),
                    Arg::with_name("value")
                        .help("The string value of the key")
                        .takes_value(true)
                        .required(true),
                ])
                .about("Set the value of a string key to a string"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(
                    Arg::with_name("key")
                        .help("A string key")
                        .takes_value(true)
                        .required(true),
                )
                .about("Get the string value of a given string key"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("set") => unimpl(),
        Some("get") => unimpl(),
        None => std::process::exit(1),
        _ => unreachable!(),
    }
}
