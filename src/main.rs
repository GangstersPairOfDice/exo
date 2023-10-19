use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
	loop{
		// use the `>>>` characters as the prompt
		// need to explicitly flush this to ensure it prints before read_line
		print!(">>> ");
		stdout().flush();

		let mut input = String::new(); // var to store user input
		stdin().read_line(&mut input).unwrap(); // reads user input, and writes into input string

		// everything after the first whitespace character
		// is interpreted as args to the command
		let mut parts = input.trim().split_whitespace();
		let command = parts.next().unwrap();
		let args = parts;
	
		let mut child = Command::new(command)
			.args(args)
			.spawn()
			.unwrap();

		// don't accept another command until this one completes
		child.wait();
	}
}
