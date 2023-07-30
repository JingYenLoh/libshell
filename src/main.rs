use std::io::{self, Write};

fn main() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    println!("{}", buffer.trim());

    main()
}
