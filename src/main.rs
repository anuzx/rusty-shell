use std::{
    io::{Write, stdin, stdout},
    process::Command,
};

fn shell_loop() {
    loop {
        print!("~$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap(); //1st element 
        let args = parts; //rest of the iterator 

        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .expect("command failed to run");

        child.wait().unwrap();
    }
}

fn main() {
    //load config files if any

    //run command loop
    shell_loop();

    //perform any shutdown or cleanup
}
