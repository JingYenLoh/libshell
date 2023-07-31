use winnow::{
    ascii::{alphanumeric0, newline, space0},
    combinator::{repeat, terminated},
    prelude::*,
};

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

pub fn program(input: &mut &str) -> PResult<Program> {
    todo!("Implement program parser")
}

pub fn command_name(input: &mut &str) -> PResult<SimpleCommand> {
    let name = alphanumeric0(input)?;
    let args: Vec<String> = repeat(0.., word).parse_next(input)?;
    Ok(SimpleCommand {
        name: name.to_string(),
        args,
    })
}

/// TODO: Follow spec for WORD
pub fn word(input: &mut &str) -> PResult<String> {
    space0.parse_next(input)?;
    alphanumeric0(input).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_word() {
        let input = "ls";
        let mut input = &input[..];
        let result = word(&mut input);
        assert_eq!(result, Ok("ls".to_string()));
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
    fn it_parses_simple_command_with_args() {
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
}
