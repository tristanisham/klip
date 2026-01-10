mod dsiplay;
use arboard::{Clipboard, SetExtLinux};
use std::io::{self, IsTerminal, Read, Write};

fn main() -> anyhow::Result<()> {
    let mut clipboard = Clipboard::new()?;

    if io::stdin().is_terminal() {
        let buf = clipboard.get_text()?;
        println!("Found {buf}");
        io::stdout().write(&buf.as_bytes())?;
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        println!("Recieved {buf}");

        clipboard.set_text(buf)?;
    }

    Ok(())
}
