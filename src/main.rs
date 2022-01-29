mod interpreter;

use std::error::Error;

use camino::Utf8PathBuf;
use clap::Parser;
use env_logger::Builder;
use interpreter::Interpreter;
use log::LevelFilter;

/// Here's my app!
#[derive(Debug, Parser)]
#[clap(name = "Rox", version)]
pub struct App {
    #[clap(long, short, global = true, parse(from_occurrences))]
    verbose: usize,

    /// optional path to file to interpret, if none is specified, REPL interpreter starts
    file_to_run: Option<Utf8PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = App::parse();

    // build logger
    let mut builder = Builder::from_default_env();
    match opts.verbose {
        0 => builder.filter_level(LevelFilter::Warn),
        1 => builder.filter_level(LevelFilter::Info),
        _ => builder.filter_level(LevelFilter::Debug),
    };
    builder.init();

    let interpreter = Interpreter {};
    if let Some(file_to_run) = opts.file_to_run {
        interpreter.run_file(file_to_run)
    } else {
        interpreter.run_prompt()
    }
}
