use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[arg(value_enum)]
    pub conversion: Conversion,
    pub input: String,
}

#[derive(Clone, ValueEnum)]
pub enum Conversion {
    Camel,
    Pascal,
    Plural,
}
