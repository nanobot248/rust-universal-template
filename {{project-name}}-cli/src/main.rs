use test_app::double;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, author)]
struct Options {
    #[clap(subcommand)]
    sub: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Double { number: i64 },
}

fn main() {
    let options = Options::parse();

    match options.sub {
        Commands::Double { number } => {
            println!("{}", double(number));
        }
    }
}
