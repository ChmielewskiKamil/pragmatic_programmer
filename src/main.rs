use std::env;
use std::process;
use turtle_language_parser::Config;
// use turtle_language_parser::print_path_for_commands;

fn main() {
    // parse_input_commands("P 2").unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arugments: {:?}", err);
    //     process::exit(1);
    // });

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = turtle_language_parser::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
