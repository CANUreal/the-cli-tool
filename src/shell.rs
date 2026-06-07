use std::io::{self, Write};
use std::process::Command;

pub fn shell() {
    loop {
        print!("%");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldnt read vro ::/(");

        let input = input.trim();

        if input.is_empty(){
            print!("HATA");
            continue;
        }

        if input == exit {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        Command::new(command)
            .args(args)
            .status()
            .unwrap_or_else( |_| {
                eprintln!("Komut bulunamadı.");
                std::process::ExitStatus::default();
            });
    }
}
