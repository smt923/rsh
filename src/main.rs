use std::collections::VecDeque;
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn shell_pwd(args: &Vec<&str>) {
    match args.len() {
        0 => println!("{}", env::current_dir().unwrap().display()),
        _ => println!("Error: pwd takes no arguments"),
    };
}

fn shell_ls(args: &Vec<&str>) {
    //TODO: expand with arguments for different formatting
    match args.len() {
        0 => {
            for e in env::current_dir()
                .unwrap()
                .read_dir()
                .expect("could not read path")
            {
                if let Ok(e) = e {
                    if e.metadata().unwrap().is_dir() {
                        print!("\\{} ", e.file_name().to_str().unwrap_or(""));
                    } else {
                        print!("{} ", e.file_name().to_str().unwrap_or(""));
                    }
                }
            }
            print!("\n");
        }
        1 => {
            let path = Path::new(args[0]);
            for e in path.canonicalize()
                .unwrap()
                .read_dir()
                .expect("could not read path")
            {
                if let Ok(e) = e {
                    if e.metadata().unwrap().is_dir() {
                        print!("\\{} ", e.file_name().to_str().unwrap_or(""));
                    } else {
                        print!("{} ", e.file_name().to_str().unwrap_or(""));
                    }
                }
            }
            print!("\n");
        }
        _ => (),
    }
}

fn shell_cd(args: &Vec<&str>) {
    if args.len() == 0 {
        println!("Error: cd requires the path to change to");
        return;
    };
    let newdir = args[0];
    let result = env::set_current_dir(newdir);
    if let Err(e) = result {
        println!("Error: Could not change directory: {}", e)
    }
}

fn shell_history(hist: &mut VecDeque<String>, args: &Vec<&str>) {
    match args.len() {
        0 => for (i, l) in hist.iter().enumerate() {
            println!("{}: {}", i, l);
        },
        1 => match args[0] {
            "-c" => {
                hist.clear();
                println!("History has been cleared!")
            }
            _ => (),
        },
        _ => println!("Invalid number of arguments"),
    }
}

fn shell_cmd(command: &str, args: &Vec<&str>) {
    match Command::new(command).args(args).spawn() {
        Err(e) => println!("error: {}", e),
        Ok(mut child) => match child.wait() {
            Err(e) => println!("error: {}", e),
            _ => {}
        },
    }
}

fn eval(input: &str, history: &mut VecDeque<String>) -> bool {
    let mut args = input.trim().split_whitespace();

    if let Some(command) = args.next() {
        let other_args = args.collect::<Vec<&str>>();

        if command != "history" {
            history.push_back(input.trim().clone().to_owned());
        }
        match command {
            "exit" => return true,
            "history" => shell_history(history, &other_args),
            "pwd" => shell_pwd(&other_args),
            "ls" => shell_ls(&other_args),
            "cd" => shell_cd(&other_args),
            _ => shell_cmd(command, &other_args),
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
