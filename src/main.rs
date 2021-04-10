use std::fs::File;
use std::io;

use flate2::read::GzDecoder;
use thiserror::Error as ThisError;

use crate::cli::parameters_from_matches;
use crate::domain::System;

mod cli;
mod distance;
mod domain;
mod edsm;
mod filter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::app().get_matches();
    let search_parameters = parameters_from_matches(&matches)?;

    let path = matches.value_of("edsm-path").unwrap();
    let compressed_file = File::open(path)?;
    let file = GzDecoder::new(compressed_file);
    let systems: Vec<Box<edsm::System>> = edsm::parse(file)?.into_iter().map(Box::from).collect();
    let filtered_system = filter::filter(&search_parameters, systems);

    display_systems(filtered_system);

    Ok(())
}

fn display_systems<T: System>(mut systems: Vec<Box<T>>) {
    systems.sort_by(|a, b| a.name().cmp(b.name()));

    for system in systems {
        println!("{}", system.name())
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("argument error: {0:?}")]
    Cli(#[from] cli::Error),
    #[error(transparent)]
    Parse(#[from] edsm::Error),
    #[error("failed to read edsm data file: {0:?}")]
    Read(#[from] io::Error),
}
