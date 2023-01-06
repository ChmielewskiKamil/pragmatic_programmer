enum Command {
    PenSelect(u32),
    PenDown,
    PenUp,
    DrawWest(u32),
    DrawEast(u32),
    DrawNorth(u32),
    DrawSouth(u32),
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod parsing_tests {
    use super::Command;

    #[test]
    fn it_should_parse_pen_select_from_str() {
        assert_eq!(parse_command("P 2"), Command::PenSelect(2));
    }
}
