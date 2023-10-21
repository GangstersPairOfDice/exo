use std::io::{stdin, stdout, Write};
use std::process::{Command, Child, Stdio};
use std::path::Path;
use std::env;

fn main() {

  // Clear the screen using the appropriate command for your operating system
  if cfg!(target_os = "windows") {
      Command::new("cmd")
          .arg("cls")
          .status()
          .expect("failed to execute process");
  } else {
      Command::new("clear")
          .status()
          .expect("failed to execute process");
    }

  loop{
		// use the `>>>` characters as the prompt
		// need to explicitly flush this to ensure it prints before read_line
		print!(">>> ");
		stdout().flush();

		let mut input = String::new(); // var to store user input

		stdin().read_line(&mut input).unwrap(); // reads user input, and writes into input string

    // must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {

  		// everything after the first whitespace character
  		// is interpreted as args to the command
  		let mut parts = command.trim().split_whitespace();
      let command = parts.next().unwrap();
  		let args = parts;

      match command {
  
        "cd" => {
          let new_dir = args.peekable().peek().map_or("/", |x| *x);
          let root = Path::new(new_dir);
          if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
          }

          previous_command = None;

        },
  
        "exit" | "quit" | "q" => return,
  
        command => {
          let stdin = previous_command
            .map_or(
              Stdio::inherit(),
              |output: Child| Stdio::from(output.stdout.unwrap())
            );

          let stdout = if commands.peek().is_some() {
            // if there is another command piped behind this one
            // prepare to send output to the next command
            Stdio::piped()
          } else {
            // otherwise, if there are no more commands piped behind this one
            // send output to shell stdout
            Stdio::inherit()
          };

          let output = Command::new(command)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn();
  
          // handle malformed user input
          match output {
            Ok(mut output) => { previous_command =  Some(output); },
            Err(e) => {
              previous_command = None;
              eprintln!("{}", e);
            },
          };
        }
      }
    }

    if let Some(mut final_command) = previous_command {
      // block until the final command has finished
      final_command.wait();
    }

  }
}
