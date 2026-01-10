use arboard::{Clipboard, SetExtLinux};
use env_logger;
use libc;
use log;
use std::io::{self, IsTerminal, Read, Write};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut clipboard = Clipboard::new()?;

    if io::stdin().is_terminal() {
        let buf = clipboard.get_text()?;
        log::debug!("(get) {buf}");
        io::stdout().write(&buf.as_bytes())?;
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        log::debug!("(set) {buf}");

        if unsafe { libc::fork() } != 0 {
            return Ok(());
        }

        clipboard.set().wait().text(&buf)?;
    }

    Ok(())
}
