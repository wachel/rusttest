use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write(b"hello world")?;

    Ok(())
}