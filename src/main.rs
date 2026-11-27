use std::{
    io::{Write, stdin, stdout},
    process::{Command, Stdio},
};

mod builtins;

fn run_pipe(input: &str) {
    let mut commands = input.trim().split('|').peekable();

    let mut previous_stdout = None;
    let mut children = Vec::new();

    while let Some(command) = commands.next() {
        let mut parts = command.split_whitespace();

        let command = match parts.next() {
            Some(command) => command,
            None => {
                eprintln!("shell: empty command in pipe");
                return;
            }
        };

        let args = parts;

        let mut cmd = Command::new(command);
        cmd.args(args);

        // Connect the previous command's output to this command's input.
        if let Some(stdout) = previous_stdout.take() {
            cmd.stdin(Stdio::from(stdout));
        }

        // Create a pipe only if another command follows.
        if commands.peek().is_some() {
            cmd.stdout(Stdio::piped());
        }

        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}: {}", command, e);
                return;
            }
        };

        previous_stdout = child.stdout.take();

        children.push(child);
    }

    // Wait after all commands have started.
    for mut child in children {
        if let Err(e) = child.wait() {
            eprintln!("wait: {}", e);
        }
    }
}

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

        if input.contains('|') {
            run_pipe(&input);
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
