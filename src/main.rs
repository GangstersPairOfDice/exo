use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;

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

    match command {

      "cd" => {
        let new_dir = args.peekable().peek().map_or("/", |x| *x);
        let root = Path::new(new_dir);
        if let Err(e) = env::set_current_dir(&root) {
          eprintln!("{}", e);
        }
      },

      "exit" => return,

      command => {
        let child = Command::new(command)
          .args(args)
          .spawn();

          // gracefully handle malformed user input

        match child {
          Ok(mut child) => { child.wait(); },
          Err(e) => eprintln!("{}", e),
        };
      }
    }
  }
}
