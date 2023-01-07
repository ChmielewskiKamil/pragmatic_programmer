use std::env;
use std::process;

use turtle_language_parser::print_path_for_commands;
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

    run(config);
}

fn run(config: Config) {
    println!("Provided instructions: \n{}", config.user_instructions);

    println!(
        "{}",
        print_path_for_commands(&config.user_instructions).unwrap()
    );
}

struct Config {
    user_instructions: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let user_instructions = args[1].clone();

        Ok(Config { user_instructions })
    }
}
