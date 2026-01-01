use clap::{Args, Parser, Subcommand};

mod sub;

mod utils;
use utils::*;

#[derive(Subcommand)]
enum Sub {
    Check(sub::check::CommandArgs),
    Create(sub::create::CommandArgs),
    Remove(sub::remove::CommandArgs),
    Dump(sub::dump::CommandArgs),
    Meta(sub::meta::CommandArgs),
    Status(sub::status::CommandArgs),
}

#[derive(Parser)]
struct CommandArgs {
    #[clap(subcommand)]
    sub: Sub,
}

fn main() {
    let args = CommandArgs::parse();

    match args.sub {
        Sub::Check(args) => sub::check::run(args),
        Sub::Create(args) => sub::create::run(args),
        Sub::Remove(args) => sub::remove::run(args),
        Sub::Dump(args) => sub::dump::run(args),
        Sub::Meta(args) => sub::meta::run(args),
        Sub::Status(args) => sub::status::run(args),
    }
}
