use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::process::Command;

use ansi_term::Color::{Blue, Green};

pub fn pwd(args: &Vec<&str>) {
	match args.len() {
		0 => println!("{}", env::current_dir().unwrap().display()),
		_ => println!("Error: pwd takes no arguments"),
	};
}

pub fn ls(args: &Vec<&str>) {
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
						print!("\\{} ", Blue.paint(e.file_name().to_str().unwrap_or("")));
					} else {
						print!("{} ", Green.paint(e.file_name().to_str().unwrap_or("")));
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

pub fn cd(args: &Vec<&str>) {
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

pub fn history(hist: &mut VecDeque<String>, args: &Vec<&str>) {
	match args.len() {
		0 => for (i, l) in hist.iter().enumerate() {
			println!("{}: {}", i, l);
		},
		1 => match args[0] {
			"-c" => {
				hist.clear();
			}
			_ => (),
		},
		_ => println!("Invalid number of arguments"),
	}
}

pub fn cmd(command: &str, args: &Vec<&str>) {
	match Command::new(command).args(args).spawn() {
		Err(e) => println!("error: {}", e),
		Ok(mut child) => match child.wait() {
			Err(e) => println!("error: {}", e),
			_ => {}
		},
	}
}
