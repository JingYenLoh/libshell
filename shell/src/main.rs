use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let mut iter = line.split_whitespace();
    let command = iter.next().unwrap();
    let args: Vec<&str> = iter.collect();

    let output = Command::new(command).args(args).output()?;
    io::stdout().write_all(output.stdout.as_slice())?;

    main()
}
