use std::collections::VecDeque;
use std::env;
use std::io;
use std::io::Write;

mod builtins;

fn eval(input: &str, history: &mut VecDeque<String>) -> bool {
    let mut args = input.trim().split_whitespace();

    if let Some(command) = args.next() {
        let other_args = args.collect::<Vec<&str>>();

        if command != "history" {
            history.push_back(input.trim().clone().to_owned());
        }
        match command {
            "exit" => return true,
            "history" => builtins::history(history, &other_args),
            "pwd" => builtins::pwd(&other_args),
            "ls" => builtins::ls(&other_args),
            "cd" => builtins::cd(&other_args),
            _ => builtins::cmd(command, &other_args),
        }
    }

    false
}

fn main() {
    let mut history: VecDeque<String> = VecDeque::with_capacity(100);
    loop {
        if let Some(name) = env::current_dir().unwrap().file_name() {
            print!("{} ~ $ ", name.to_str().unwrap_or("UNKNOWN"));
        } else {
            print!("$ ");
        };
        io::stdout().flush().ok().expect("Could not flush");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Could not read line");

        if eval(&input, &mut history) {
            break;
        }
    }
}
