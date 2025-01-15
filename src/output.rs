use std::fs::File;
use std::io::{stdout, Stdout, Write};
use crate::error::Result;

pub enum Output {
    Stdout(Stdout),
    File(File),
}

impl Output {
    pub fn try_from_args(arg: &Option<String>) -> Result<Self> {
        Ok(match arg {
            None => Self::Stdout(stdout()),
            Some(path) => Self::File(File::create(path)?),
        })
    }

    pub fn write(&mut self, s: &str) -> Result<()> {
        match self {
            Self::Stdout(f) => writeln!(f, "{}", s)?,
            Self::File(f) => writeln!(f, "{}", s)?,
        };
        Ok(())
    }
}