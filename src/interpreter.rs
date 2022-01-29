use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read, Write},
};

use camino::Utf8PathBuf;

pub struct Interpreter {}

impl Interpreter {
    pub fn run_file(&self, file_path: Utf8PathBuf) -> Result<(), Box<dyn Error>> {
        let f = File::open(file_path)?;
        let mut buffer = String::new();
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut buffer)?;
        self.run(&buffer)
    }

    pub fn run_prompt(&self) -> Result<(), Box<dyn Error>> {
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
                a => self.run(a)?,
            }
        }
        Ok(())
    }

    fn run(&self, buffer: &str) -> Result<(), Box<dyn Error>> {
        println!("{}", buffer);
        Ok(())
    }
}
