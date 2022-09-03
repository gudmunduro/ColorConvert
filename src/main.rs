pub mod parser;
pub mod formatter;

use std::process::exit;

use clap::{Parser, ValueEnum};
use parser::parse_color;

use crate::formatter::format_color;

#[derive(Parser, Debug)]
#[clap(version="0.1.0", about="Convert between different color formats", long_about = None)]
struct Args {
    // What format to convert to
    #[clap(arg_enum, value_parser)]
    format: TargetFormats,
    
    // Color value
    #[clap(value_parser)]
    input: String
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TargetFormats {
    Intrgb,
    Intrgba,
    Floatrgb,
    Floatrgba,
    Hexrgb,
    Hexrgba
}

fn main() {
    let args = Args::parse();

    let color = match parse_color(&args.input) {
        Ok(c) => c,
        Err(_) => {
            println!("Invalid format for input color");
            exit(1);
        }
    };

    println!("{}", format_color(&args.format, &color));
}
