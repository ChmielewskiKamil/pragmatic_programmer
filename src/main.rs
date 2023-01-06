use pragmatic_programmer::parse_single_command;
use std::process;

fn main() {
    parse_single_command("P 2").unwrap_or_else(|err| {
        println!("Problem parsing arugments: {:?}", err);
        process::exit(1);
    });
}
