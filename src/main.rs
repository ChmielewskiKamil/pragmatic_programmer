// use std::process;
use std::env;
use turtle_language_parser::print_path_for_commands;

fn main() {
    // parse_input_commands("P 2").unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arugments: {:?}", err);
    //     process::exit(1);
    // });

    let args: Vec<String> = env::args().collect();
    let instructions = parse_config(&args);
}

fn parse_config(args: &[String]) -> &str {
    let user_instructions = &args[1];

    user_instructions
}
