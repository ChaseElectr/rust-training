use kvs::{KvStore, Result};
use structopt::StructOpt;

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
    #[structopt(name = "rm")]
    /// Remove the value of a given string key
    Remove {
        #[structopt(required = true)]
        /// The key to be removed
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
    let mut store = KvStore::open("./")?;

    match opt.cmd {
        Command::Set { key, value } => store.set(key, value),
        Command::Get { key } => {
            let value = store
                .get(key)?
                .or_else(|| Some(String::from("Key not found")));
            println!("{}", value.unwrap());
            Ok(())
        }
        Command::Remove { key } => store.remove(key),
    }
}
