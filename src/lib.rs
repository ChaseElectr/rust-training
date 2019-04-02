use clap::{App, Arg, SubCommand};

#[derive(Default)]
pub struct KvStore;

fn unimpl() {
    eprintln!("unimplemented");
    std::process::exit(1);
}

impl KvStore {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn set(&mut self, key: String, value: String) {
        unimplemented!();
    }

    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!();
    }
}

pub fn run() {
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
