use clap::Parser;
use cli::{Cli, Conversion};
use inflector::Inflector;

mod cli;

fn main() {
    let cli = Cli::parse();
    let output = match cli.conversion {
        Conversion::Camel => cli.input.to_camel_case(),
        Conversion::Pascal => cli.input.to_pascal_case(),
        Conversion::Plural => cli.input.to_plural(),
    };

    print!("{output}");
}
