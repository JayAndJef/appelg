pub mod parser;

use std::{env};

use parser::get_file_reader;

use crate::parser::parse_reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2  => {
            println!("called with 2 arguments, filename is {}", args[1]);
            let reader = get_file_reader(args[1].clone())
                .expect("file not found");

            println!("{}", parse_reader(reader).unwrap());
        }, // Input file, assume output file
        3 => todo!(),
        _ => eprintln!("Error: Either specify infile or both infile and outfile.")
    }
}
