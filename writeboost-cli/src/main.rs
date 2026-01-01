use clap::{Parser, Args, Subcommand};

mod sub;

mod utils;
use utils::*;

#[derive(Subcommand)]
enum Sub {
    Check(sub::check::Opts),
    Create(sub::create::Opts),
    Remove(sub::remove::Opts),
    Dump(sub::dump::Opts),
    Meta(sub::meta::Opts),
    Status(sub::status::Opts),
}

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    sub: Sub,
}

fn main() {
    let opts = Opts::parse();

    match opts.sub {
        Sub::Check(args) => sub::check::run(args),
        Sub::Create(args) => sub::create::run(args),
        Sub::Remove(args) => sub::remove::run(args),
        Sub::Dump(args) => sub::dump::run(args),
        Sub::Meta(args) => sub::meta::run(args),
        Sub::Status(args) => sub::status::run(args),
    }
}