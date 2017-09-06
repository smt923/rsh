extern crate chrono;

use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::process::Command;
use std::io::Error;
use std::fs::DirEntry;

use self::chrono::offset::Utc;
use self::chrono::DateTime;

use ansi_term::Color::{Blue, Cyan, Green};

#[inline]
fn fileprint(f: Result<DirEntry, Error>) {
	if let Ok(f) = f {
		if let Ok(meta) = f.metadata() {
			if meta.is_dir() {
				print!("\\{} ", Blue.paint(f.file_name().to_str().unwrap_or("")),);
			} else if meta.file_type().is_symlink() {
				print!("{} ", Cyan.paint(f.file_name().to_str().unwrap_or("")),);
			} else {
				print!("{} ", Green.paint(f.file_name().to_str().unwrap_or("")),);
			}
		}
	}
}

#[inline]
fn fileprintl(f: Result<DirEntry, Error>) {
	if let Ok(f) = f {
		if let Ok(meta) = f.metadata() {
			let fsize = meta.len();
			let fmod: DateTime<Utc> = meta.modified().unwrap().into();
			if meta.is_dir() {
				println!(
					"{: >8} {: >6} - \\{: >6} ",
					fsize,
					fmod.format("%b %m %R"),
					Blue.paint(f.file_name().to_str().unwrap_or("")),
				);
			} else if meta.file_type().is_symlink() {
				println!(
					"{: >8} {: >6} - {: >6} ",
					fsize,
					fmod.format("%b %m %R"),
					Cyan.paint(f.file_name().to_str().unwrap_or("")),
				);
			} else {
				println!(
					"{: >8} {: >6} - {: >6} ",
					fsize,
					fmod.format("%b %m %R"),
					Green.paint(f.file_name().to_str().unwrap_or("")),
				);
			}
		}
	}
}

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
				fileprint(e);
			}
			print!("\n");
		}
		1 => if args[0] == "-l" {
			for e in env::current_dir()
				.unwrap()
				.read_dir()
				.expect("could not read path")
			{
				fileprintl(e);
			}
		} else {
			let path = Path::new(args[0]);
			for e in path.canonicalize()
				.unwrap()
				.read_dir()
				.expect("could not read path")
			{
				fileprint(e);
			}
			print!("\n");
		},
		2 => match args[0] {
			"-l" => {
				let path = Path::new(args[1]);
				for e in path.canonicalize()
					.unwrap()
					.read_dir()
					.expect("could not read path")
				{
					fileprintl(e);
				}
			}
			_ => println!("Error: usage: ls <args> <path>"),
		},
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
