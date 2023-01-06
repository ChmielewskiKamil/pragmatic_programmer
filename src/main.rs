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

fn parse_single_command(command: &str) -> Command {
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
        'U' => Command::PenUp,
        'S' => Command::DrawSouth(parameter.unwrap()),
        'N' => Command::DrawNorth(parameter.unwrap()),
        'W' => Command::DrawWest(parameter.unwrap()),
        'E' => Command::DrawEast(parameter.unwrap()),
        _ => panic!("Encountered error while parsing command"),
    }
}

fn main() {
    parse_single_command("P 2");
}

#[cfg(test)]
mod parsing_tests {
    use super::{parse_single_command, Command};

    #[test]
    fn it_should_parse_pen_select() {
        assert_eq!(parse_single_command("P 2"), Command::PenSelect(2));
    }

    #[test]
    fn it_should_parse_pen_down() {
        assert_eq!(parse_single_command("D"), Command::PenDown);
    }

    #[test]
    fn it_should_parse_with_trailing_spaces() {
        assert_eq!(parse_single_command(" P 2 "), Command::PenSelect(2));
    }

    #[test]
    fn it_should_parse_pen_up() {
        assert_eq!(parse_single_command("U"), Command::PenUp);
    }

    #[test]
    fn it_should_parse_draw_south() {
        assert_eq!(parse_single_command("S 5"), Command::DrawSouth(5));
    }

    #[test]
    fn it_should_parse_draw_north() {
        assert_eq!(parse_single_command("N 10"), Command::DrawNorth(10));
    }

    #[test]
    fn it_should_parse_draw_west() {
        assert_eq!(parse_single_command("W 20"), Command::DrawWest(20));
    }

    #[test]
    fn it_should_parse_draw_east() {
        assert_eq!(parse_single_command("E 35"), Command::DrawEast(35));
    }
}
