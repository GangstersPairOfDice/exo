use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
	loop{
		// use the `>>>` characters as the prompt
		// need to explicitly flush this to ensure it prints before read_line
		print!(">>> ");
		stdout().flush();

		let mut input = String::new(); // creates new string to hold user input
		stdin().read_line(&mut input).unwrap(); // read user input, and writes user input when the enter key is pressed
	
		// read_line leaves a trailing newline ( \n ), which trim removes
		let command = input.trim();
	
		let mut child = Command::new(command)
			.spawn()
			.unwrap();

		// don't accept another command until this one completes
		child.wait();
	}
}
