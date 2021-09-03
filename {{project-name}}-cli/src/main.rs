use {{crate_name}}::double;
use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Double { number: i32 },
}

fn main() {
    let options = Options::parse();

    match options.sub {
        SubCommand::Double { number } => {
            println!("{}", double(number));
        }
    }
}
