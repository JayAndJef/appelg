pub mod parser;

use std::{env};

use parser::{get_file_reader, parse_reader};

use argh::FromArgs;

fn default_input() -> String {
    "appelinput.txt".to_string()
}

/// Parse a file into an appel replay
#[derive(FromArgs)]
struct AppelH {
    /// optional input file path 
    #[argh(option, default = "default_input()")]
    input_file: String
}

fn main() {
    let args: AppelH = argh::from_env();
    let reader = get_file_reader(args.input_file)
                .expect("file not found");
    match parse_reader(reader) {
        Ok(o) => println!("{}", o),
        Err(err) => println!("{:?}", err),
    }
}
