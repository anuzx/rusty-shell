# rusty-shell

A lightweight Unix shell written in Rust for daily use. It supports built-in commands, command arguments, pipes, and basic error handling.

## Features

- **Built-in commands:** `cd`, `echo`, `exit`, and `cat`
- **External commands:** Run programs such as `ls`, `nvim`, and `grep`
- **Pipes:** Pass the output of one command to another using `|`
- **Command arguments:** Pass arguments to built-in and external commands
- **Error handling:** Display errors when a command fails or cannot be started

## Getting Started

### Prerequisites

- Rust and Cargo
- A Unix-like operating system

### Run the Shell

Clone the repository:

```bash
git clone <repository-url>
cd <repository-directory>
```

Start the shell:

```bash
cargo run
```

## Usage

Run commands as you would in a regular shell:

```bash
ls
cat file.txt
echo Hello, world!
```

Pass arguments to commands:

```bash
nvim file.txt
```

Use pipes to connect commands:

```bash
cat file.txt | grep "hello"
```

Change the current directory:

```bash
cd ..
```

Exit the shell:

```bash
exit
```

