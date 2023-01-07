// use std::process;
use std::env;
// use turtle_language_parser::print_path_for_commands;

fn main() {
    // parse_input_commands("P 2").unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arugments: {:?}", err);
    //     process::exit(1);
    // });

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!("Provided instructions: \n{}", config.user_instructions);
}

struct Config {
    user_instructions: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let user_instructions = args[1].clone();

        Config { user_instructions }
    }
}
