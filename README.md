# exo

exo is a shell written in Rust.

## Installation

First, make sure you have [Rust](https://www.rust-lang.org/) installed. You can install it from [here](https://www.rust-lang.org/learn/get-started)

## Usage

Once you have Rust installed, go into the exo folder, and use:

```bash
cargo run
```

Once and if it compiles correctly, you should see:

```bash
>>> 
```

This means that exo is running. You can now do basic commands such as:

```bash
# returns computer files and directories in current directory
ls

# changes directory
cd <directory>

# pipington, uses previous commands output as input to the next command
# lists files and directories, and filters by whatever string you want
ls | grep <string>

# terminates exo session
`exit` or `quit` or `q`
```

More features are available, fx. already build-in shell commands, like `echo`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

None at the moment
