use clap::{StructOpt, Subcommand};

use self::{bundle::BundleCommand, minify::MinifyCommand, test::TestCommand};

mod bundle;
mod minify;
mod test;

#[derive(Debug, clap::Parser)]
struct AppArgs {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    #[clap(flatten)]
    Bundle(BundleCommand),
    #[clap(flatten)]
    Minify(MinifyCommand),
    #[clap(flatten)]
    Test(TestCommand),
}

fn main() {
    let args = AppArgs::parse();

    match args.cmd {
        Cmd::Bundle(_) => todo!(),
        Cmd::Minify(_) => todo!(),
        Cmd::Test(_) => todo!(),
    }
}
