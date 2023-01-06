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
    // tokens will be used to derive the command and a parameter from str command
    // the command is formatted like this "P 2"
    // that's why it is necessary to split two tokens on the whitespace
    let tokens: Vec<&str> = command.split_whitespace().collect();
    // first char is the command type
    // as tokens[0] is of type string and we are interested in chars
    // we need to convert it with chars()
    let c = tokens[0].chars().next().unwrap();
    // the integer after the command is the parameter
    let parameter = tokens.get(1).and_then(|s| s.parse().ok());
    match c {
        'P' => Command::PenSelect(parameter.unwrap()),
        'D' => Command::PenDown,
        _ => panic!("Encountered error while parsing command"),
    }
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

    #[test]
    fn it_should_parse_with_trailing_spaces() {
        assert_eq!(parse_command(" P 2 "), Command::PenSelect(2));
    }

    #[test]
    fn it_should_parse_pen_up_from_str() {
        assert_eq!(parse_command("U"), Command::PenUp);
    }
}
