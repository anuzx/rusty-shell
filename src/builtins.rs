use std::{env, fs};

pub fn shell_cd(args: &[&str]) {
    let target = match args.first() {
        Some(path) => *path,
        None => match env::var("HOME") {
            Ok(home) => {
                if let Err(e) = env::set_current_dir(home) {
                    eprintln!("cd: {}", e);
                }
                return;
            }
            Err(e) => {
                eprintln!("cd: HOME is not set: {}", e);
                return;
            }
        },
    };

    if let Err(e) = env::set_current_dir(target) {
        eprintln!("cd: {}", e);
    }
}

pub fn shell_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

pub fn shell_cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: missing file operand");
        return;
    }

    for filename in args {
        match fs::read_to_string(filename) {
            Ok(contents) => {
                print!("{}", contents);
            }
            Err(e) => {
                eprintln!("cat: {}: {}", filename, e);
            }
        }
    }
}
