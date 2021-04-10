use std::num::ParseIntError;
use std::str::FromStr;

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use thiserror::Error as ThisError;

use crate::domain::SearchOptions;

pub fn app() -> App<'static> {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("edsm-path")
                .about("This is the path to th EDSM dump")
                .required(true),
        )
        .arg(
            Arg::new("min-docks-large")
                .about(
                    "Filter the systems that are have less than the given number of docks with room for large ships",
                )
                .long("min-docks-large")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
        .arg(
            Arg::new("min-docks")
                .about(
                    "Filter the systems that are have less than the given number of docks with room for ships"
                )
                .long("min-docks")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
}

pub fn parameters_from_matches(matches: &ArgMatches) -> Result<SearchOptions, Error> {
    Ok(SearchOptions {
        min_large_docks: matches
            .value_of("min-docks-large")
            .map(|value| usize::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(Some))?,
        min_docks: matches
            .value_of("min-docks")
            .map(|value| usize::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(Some))?,
    })
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("invalid count: {0:?}")]
    InvalidCount(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use crate::cli::{app, parameters_from_matches};
    use crate::domain::SearchOptions;

    #[test]
    fn no_switches() {
        let args = app().get_matches_from(vec!["ed-system-search", "some-edsm-dump.json.gz"]);
        assert_eq!(
            parameters_from_matches(&args).unwrap(),
            SearchOptions {
                min_large_docks: None,
                min_docks: None
            }
        )
    }

    #[test]
    fn large_docks_invalid() {
        let args = app().get_matches_from(vec![
            "ed-system-search",
            "--min-docks-large=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(parameters_from_matches(&args).is_err(), true)
    }

    #[test]
    fn large_docks_present() {
        let args = app().get_matches_from(vec![
            "ed-system-search",
            "--min-docks-large=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args).unwrap(),
            SearchOptions {
                min_large_docks: Some(10),
                min_docks: None
            }
        )
    }

    #[test]
    fn docks_invalid() {
        let args = app().get_matches_from(vec![
            "ed-system-search",
            "--min-docks=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(parameters_from_matches(&args).is_err(), true)
    }

    #[test]
    fn docks_present() {
        let args = app().get_matches_from(vec![
            "ed-system-search",
            "--min-docks=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args).unwrap(),
            SearchOptions {
                min_large_docks: None,
                min_docks: Some(10)
            }
        )
    }
}
