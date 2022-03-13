use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use camino::Utf8PathBuf;

use crate::{ast::visitor::ASTPrinter, error::*, parser::Parser, scanner::Scanner};

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

                    if let Err(err) = r {
                        eprintln!("{}", err);
                        self.reset_error();
                    }
                }
            }
        }
        Ok(())
    }

    fn process_result<T>(&mut self, result: Result<T, InternalRoxError>) -> FacingRoxResult<T> {
        result.map_err(|e| {
            self.had_error = true;
            eprintln!("{}", e);
            match e {
                InternalRoxError::SyntaxError { .. } => FacingRoxError::SyntaxError,
            }
        })
    }
    fn process_result_vec<T>(
        &mut self,
        result: Result<T, Vec<InternalRoxError>>,
    ) -> FacingRoxResult<T> {
        result.map_err(|errs| {
            self.had_error = true;
            for e in &errs {
                eprintln!("{}", e);
            }
            match errs[0] {
                InternalRoxError::SyntaxError { .. } => FacingRoxError::SyntaxError,
            }
        })
    }

    fn reset_error(&mut self) {
        self.had_error = false;
    }

    fn run(&mut self, buffer: &str) -> FacingRoxResult<()> {
        let scanner = Scanner::new(buffer);
        let tokens = self.process_result_vec(scanner.scan_tokens())?;

        if self.had_error {}

        let p = Parser::new(tokens);
        let ast = p.parse();

        println!("{}", ASTPrinter {}.print(ast));

        Ok(())
    }
}
