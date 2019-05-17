use kvs::KvStore;
use kvs::Result;
use structopt::StructOpt;

fn unimpl() {
    eprintln!("unimplemented");
    std::process::exit(1);
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "set")]
    /// Set the value of a string key to a string
    Set {
        #[structopt(required = true)]
        /// A string key
        key: String,
        #[structopt(required = true)]
        /// The string value of the key
        value: String,
    },
    #[structopt(name = "get")]
    /// Get the string value of a given string key
    Get {
        #[structopt(required = true)]
        /// A string key
        key: String,
    },
}

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"))]
#[structopt(raw(setting = "structopt::clap::AppSettings::SubcommandRequiredElseHelp"))]
#[structopt(raw(setting = "structopt::clap::AppSettings::VersionlessSubcommands"))]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut datas = KvStore::new();

    match opt.cmd {
        Command::Set { key, value } => datas.set(key, value),
        Command::Get { key } => {
            let value = datas
                .get(key)?
                .or_else(|| Some(String::from("Key not found")));
            println!("{}", value.unwrap());
            Ok(())
        }
    }
}
