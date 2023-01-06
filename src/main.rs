#[derive(Debug, PartialEq)]
enum Command {
    PenSelect(u32),
    PenDown,
    PenUp,
    DrawWest(u32),
    DrawEast(u32),
    DrawNorth(u32),
    DrawSouth(u32),
}

fn parse_command(command: &str) -> Command {
    Command::PenSelect(2)
}

fn main() {
    parse_command("P 2");
}

#[cfg(test)]
mod parsing_tests {
    use super::{parse_command, Command};

    #[test]
    fn it_should_parse_pen_select_from_str() {
        assert_eq!(parse_command("P 2"), Command::PenSelect(2));
    }

    #[test]
    fn it_should_parse_pen_down_from_str() {
        assert_eq!(parse_command("D"), Command::PenDown);
    }
}
