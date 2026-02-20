use std::io::{self, Write};

pub fn confirm(prompt: &str) -> anyhow::Result<bool> {
    loop {
        print!("{prompt} [y/n]: ");
        io::stdout().flush()?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;

        match s.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => {}
        }
    }
}
