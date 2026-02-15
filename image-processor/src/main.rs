use clap::Parser;

mod args;

fn main() {
    let _ = args::Args::parse();
}
