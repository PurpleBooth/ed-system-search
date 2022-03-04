//! A commandline tool to search for searching through systems for suitable systems for your Squadron

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

use std::fs::File;
use std::io;

use crate::cli::parameters_from_matches;
use crate::cli::Cli;
use crate::domain::System;
use flate2::read::GzDecoder;
use miette::{set_panic_hook, IntoDiagnostic, Result};
use thiserror::Error as ThisError;

mod cli;
mod distance;
mod domain;
mod edsm;
mod filter;
mod stub;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    set_panic_hook();
    let args = Cli::parse();
    let compressed_file = File::open(&args.edsm_path).into_diagnostic()?;
    let file = GzDecoder::new(compressed_file);
    let systems = edsm::parse(file).into_diagnostic()?;

    let search_parameters = parameters_from_matches(&args, systems.as_slice()).into_diagnostic()?;
    let filtered_system = filter::filter(&search_parameters, systems.as_slice());

    display_systems(filtered_system.await);

    Ok(())
}

fn display_systems<'a, T: System<'a>>(systems: Vec<T>) {
    for system in systems {
        println!("{}", system.name());
    }
}

/// General error type
#[derive(ThisError, Debug)]
pub enum Error {
    /// Error from parsing arguments or similar
    #[error("argument error: {0:?}")]
    Cli(#[from] cli::Error),
    /// Error from parsing edsm file
    #[error(transparent)]
    Parse(#[from] edsm::Error),
    /// Error from reading edsm file
    #[error("failed to read edsm data file: {0:?}")]
    Read(#[from] io::Error),
}
