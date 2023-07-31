use winnow::{
    ascii::alphanumeric0,
    combinator::{alt, fail, peek, repeat, success},
    dispatch,
    prelude::*,
    stream::{Stream, AsChar},
    token::{tag, take_while, take_till0, take_till1},
};

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    // &&
    AndIf,
    // ||
    OrIf,
    // ;;
    DSemi,
    // <<
    DLess,
    // >>
    DGreat,
    // <&
    LessAnd,
    // >&
    GreatAnd,
    // <>
    LessGreat,
    // <<-
    DLessDash,
    // >|
    Clobber,
}

/// simple_command: cmd_prefix cmd_word cmd_suffix (TODO)
///               | cmd_prefix cmd_word (TODO)
///               | cmd_prefix (TODO)
///               | cmd_name cmd_suffix (TODO)
///               | cmd_name
#[derive(Debug, PartialEq, Clone)]
pub struct SimpleCommand {
    pub name: String,
    pub args: Vec<String>,
}

pub struct Program {
    pub commands: Vec<SimpleCommand>,
}

fn program(input: &mut &str) -> PResult<Program> {
    todo!("Implement program parser")
}

fn command_name(input: &mut &str) -> PResult<SimpleCommand> {
    let name = word(input)?;
    let args: Vec<String> = repeat(0.., word).parse_next(input).unwrap_or(vec![]);
    Ok(SimpleCommand {
        name: name.to_string(),
        args,
    })
}

/// TODO: Follow spec for WORD
fn word(input: &mut &str) -> PResult<String> {
    // EOS
    if input.is_empty() {
        return fail(input);
    }
    // eat all spaces
    take_while(0.., AsChar::is_space).parse_next(input)?;
    take_till1(|c| {
        c == ' ' || c == '"' || c == '\'' || c == '`' || c == '(' || c == ')' || c == '<'
            || c == '>' || c == '|' || c == '&' || c == ';' || c == '\n'
    }).parse_next(input).map(|s| s.to_string())
}

fn and_if(input: &mut &str) -> PResult<Operator> {
    tag("&&").parse_next(input).map(|_| Operator::AndIf)
}

fn or_if(input: &mut &str) -> PResult<Operator> {
    tag("||").parse_next(input).map(|_| Operator::OrIf)
}

fn d_semi(input: &mut &str) -> PResult<Operator> {
    tag(";;").parse_next(input).map(|_| Operator::DSemi)
}

fn d_less(input: &mut &str) -> PResult<Operator> {
    tag("<<").parse_next(input).map(|_| Operator::DLess)
}

fn d_great(input: &mut &str) -> PResult<Operator> {
    tag(">>").parse_next(input).map(|_| Operator::DGreat)
}

fn less_and(input: &mut &str) -> PResult<Operator> {
    tag("<&").parse_next(input).map(|_| Operator::LessAnd)
}

fn great_and(input: &mut &str) -> PResult<Operator> {
    tag(">&").parse_next(input).map(|_| Operator::GreatAnd)
}

fn less_great(input: &mut &str) -> PResult<Operator> {
    tag("<>").parse_next(input).map(|_| Operator::LessGreat)
}

fn d_less_dash(input: &mut &str) -> PResult<Operator> {
    tag("<<-").parse_next(input).map(|_| Operator::DLessDash)
}

fn clobber(input: &mut &str) -> PResult<Operator> {
    tag(">|").parse_next(input).map(|_| Operator::Clobber)
}

fn operator(input: &mut &str) -> PResult<Operator> {
    alt((
        d_less_dash,
        and_if,
        or_if,
        d_semi,
        d_less,
        d_great,
        less_and,
        great_and,
        less_great,
        clobber,
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_operator_and() {
        let input = "&&";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::AndIf));
    }

    #[test]
    fn it_parses_operator_or() {
        let input = "||";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::OrIf));
    }

    #[test]
    fn it_parses_operator_d_semi() {
        let input = ";;";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::DSemi));
    }

    #[test]
    fn it_parses_operator_d_less() {
        let input = "<<";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::DLess));
    }

    #[test]
    fn it_parses_operator_d_great() {
        let input = ">>";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::DGreat));
    }

    #[test]
    fn it_parses_operator_less_and() {
        let input = "<&";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::LessAnd));
    }

    #[test]
    fn it_parses_operator_great_and() {
        let input = ">&";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::GreatAnd));
    }

    #[test]
    fn it_parses_operator_less_great() {
        let input = "<>";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::LessGreat));
    }

    #[test]
    fn it_parses_operator_d_less_dash() {
        let input = "<<-";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::DLessDash));
    }

    #[test]
    fn it_parses_operator_clobber() {
        let input = ">|";
        let mut input = &input[..];
        let result = operator(&mut input);
        assert_eq!(result, Ok(Operator::Clobber));
    }

    #[test]
    fn it_parses_word() {
        let input = "ls";
        let mut input = &input[..];
        let result = word(&mut input);
        assert_eq!(result, Ok("ls".to_string()));
    }

    #[test]
    fn it_parses_simple_command_nonalphanumeric() {
        let input = "xdg-open";
        let mut input = &input[..];
        let result = command_name(&mut input);
        assert_eq!(
            result,
            Ok(SimpleCommand {
                name: "xdg-open".to_string(),
                args: vec![],
            })
        );
    }

    #[test]
    fn it_parses_simple_command() {
        let input = "ls";
        let mut input = &input[..];
        let result = command_name(&mut input);
        assert_eq!(
            result,
            Ok(SimpleCommand {
                name: "ls".to_string(),
                args: vec![],
            })
        );
    }

    #[test]
    fn it_parses_simple_command_args() {
        let input = "ls -l";
        let mut input = &input[..];
        let result = command_name(&mut input);
        assert_eq!(
            result,
            Ok(SimpleCommand {
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            })
        );
    }

    #[test]
    fn it_parses_simple_command_with_trailing_whitespace() {
        let input = "ls        ";
        let mut input = &input[..];
        let result = command_name(&mut input);
        assert_eq!(
            result,
            Ok(SimpleCommand {
                name: "ls".to_string(),
                args: vec![],
            })
        );
    }


    #[test]
    fn it_parses_simple_command_args_with_trailing_whitespace() {
        let input = "ls        -la           ";
        let mut input = &input[..];
        let result = command_name(&mut input);
        assert_eq!(
            result,
            Ok(SimpleCommand {
                name: "ls".to_string(),
                args: vec!["-la".to_string()],
            })
        );
    }

}
