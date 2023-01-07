// use std::process;
// use turtle_language_parser::print_path_for_commands;
use std::env;

fn main() {
    // parse_input_commands("P 2").unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arugments: {:?}", err);
    //     process::exit(1);
    // });

    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
