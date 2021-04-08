mod cli;
mod edsm;
use std::fs::File;
use std::io;
use thiserror::Error as ThisError;

fn main() -> Result<(), Error> {
    let matches = cli::app().get_matches();
    let file = File::open(matches.value_of("edsm-path").unwrap())?;
    edsm::parse(file)?;

    Ok(())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse edsm data")]
    Parse(#[from] edsm::Error),
    #[error("failed to read edsm data file")]
    Read(#[from] io::Error),
}
