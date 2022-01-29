use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use camino::Utf8PathBuf;

use crate::error::*;

#[derive(Default)]
pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn run_file(&mut self, file_path: Utf8PathBuf) -> RoxResult<()> {
        let f = File::open(file_path)?;
        let mut buffer = String::new();
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut buffer)?;
        self.run(&buffer)
    }

    pub fn run_prompt(&mut self) -> RoxResult<()> {
        let stdin = io::stdin(); // We get `Stdin` here.

        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut buffer = String::new();
            let read = stdin.read_line(&mut buffer)?;

            if read == 0 {
                // user entered C^D
                break;
            }

            match buffer.trim_end() {
                "exit" | "exit()" | "quit" | "quit()" => break,
                a => {
                    let r = self.run(a);
                    if let Err(err @ RoxError::SyntaxError { .. }) = r {
                        eprintln!("{}", err);
                        self.reset_error();
                    }
                }
            }
        }
        Ok(())
    }

    fn emit_syntax_error(&mut self, line: usize, message: String) -> RoxError {
        self.had_error = true;
        RoxError::SyntaxError { line, message }
    }

    fn reset_error(&mut self) {
        self.had_error = false;
    }

    fn run(&mut self, buffer: &str) -> RoxResult<()> {
        match buffer {
            "crash" => Err(self.emit_syntax_error(0, "blablabla".into())),
            _ => {
                println!("{}", buffer);
                Ok(())
            }
        }
    }
}
