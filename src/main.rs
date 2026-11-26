use std::{
    io::{Write, stdin, stdout},
    process::Command,
};

mod builtins;

fn main() {
    loop {
        print!("~$ ");
        stdout().flush().unwrap();

        let mut input = String::new();

        if stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" => return,

            "cd" => {
                builtins::shell_cd(args);
            }

            "cat" => {
                builtins::shell_cat(args);
            }

            "echo" => {
                builtins::shell_echo(args);
            }

            command => {
                let mut child = match Command::new(command).args(args).spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        eprintln!("{}: {}", command, e);
                        continue;
                    }
                };

                if let Err(e) = child.wait() {
                    eprintln!("wait: {}", e);
                }
            }
        }
    }
}
