use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io, process};

pub struct Config {
    pub from: PathBuf,
    pub to: PathBuf,
    pub overwrite: bool,
}

impl Config {
    pub fn from_strings(from: String, to: String) -> Result<Self, Box<dyn Error>> {
        Ok(Config {
            from: PathBuf::from(from),
            to: PathBuf::from(to),
            overwrite: false,
        })
    }
}

pub trait Fatal<T> {
    fn or_fatal(self) -> T;
}

impl<T, E: Display> Fatal<T> for Result<T, E> {
    fn or_fatal(self) -> T {
        match self {
            Ok(res) => res,
            Err(err) => fatal_err(err),
        }
    }
}

pub fn run(cfg: &Config) -> io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create(cfg.overwrite)
        .create_new(!cfg.overwrite)
        .open(&cfg.to)?
        .flush()?;

    fs::copy(&cfg.from, &cfg.to)?;
    Ok(())
}

pub fn fatal_err(err: impl Display) -> ! {
    eprintln!("Fatal: {err}");
    process::exit(1);
}
