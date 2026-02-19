use std::io::{self, Write};

pub fn confirm(prompt: &str) -> bool {
    loop {
        print!("{prompt} [y/n]: ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        match s.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {}
        }
    }
}
