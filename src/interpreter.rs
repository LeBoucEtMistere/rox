use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read, Write},
};

use camino::Utf8PathBuf;

use crate::{ast::visitor::ASTPrettyPrinter, error::*, parser::Parser, scanner::Scanner};

#[derive(Default)]
pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn run_file(&mut self, file_path: Utf8PathBuf) -> FacingRoxResult<()> {
        let f = File::open(file_path)?;
        let mut buffer = String::new();
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut buffer)?;
        self.run(&buffer)
            .map_err(|err_vec| err_vec.into_iter().nth(1).unwrap())
    }

    pub fn run_prompt(&mut self) -> FacingRoxResult<()> {
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

                    if r.is_err() {
                        self.reset_error();
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_errors<T, E>(&mut self, result: Result<T, Vec<E>>) -> FacingRoxResults<T>
    where
        E: Into<FacingRoxError> + Error,
    {
        result.map_err(|errs| {
            self.had_error = true;
            errs.into_iter()
                .map(|err| {
                    eprintln!("{}", err);
                    err.into()
                })
                .collect()
        })
    }

    fn reset_error(&mut self) {
        self.had_error = false;
    }

    fn run(&mut self, buffer: &str) -> FacingRoxResults<()> {
        let scanner = Scanner::new(buffer);
        let tokens = self.handle_errors(scanner.scan_tokens())?;

        let p = Parser::new(tokens);
        let ast = self.handle_errors(p.parse())?;

        println!("{}", ASTPrettyPrinter::new().print(&ast));

        Ok(())
    }
}
