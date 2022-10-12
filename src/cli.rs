use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;

use clap::Parser;
use thiserror::Error as ThisError;

use crate::domain;
use crate::domain::{
    allegiance, exclude_permit_locked, exclude_player_faction, exclude_rare_commodity, government,
    max_distance_from_reference, max_distance_from_sol, max_number_of_factions, min_docks,
    min_large_docks, min_population, min_starports, System,
};

#[derive(Parser, Debug, Default, PartialEq)]
#[clap(author, version, about)]
pub struct Cli {
    /// This is the path to th EDSM dump in .json.gz format
    #[clap(value_name = "edsm-path")]
    pub edsm_path: PathBuf,
    /// Filter the systems that are have less than the given number of docks with room for large ships
    #[clap(long, value_name = "COUNT")]
    min_docks_large: Option<usize>,
    /// Filter by allegiance
    #[clap(long, value_name = "MAJOR_FACTION")]
    allegiance: Option<String>,
    /// Filter by government
    #[clap(long, value_name = "GOVERNMENT_TYPE")]
    government: Option<String>,
    /// Filter the systems that are have less than the given number of starports
    #[clap(long, value_name = "COUNT")]
    min_starports: Option<usize>,
    /// Filter the systems that are have less than the given number of docks
    #[clap(long, value_name = "COUNT")]
    min_docks: Option<usize>,
    /// Filter the systems that are have less than the given population
    #[clap(long, value_name = "COUNT")]
    min_population: Option<u128>,
    /// Filter the systems that are further than this distance from sol
    #[clap(long, value_name = "LIGHT_SECONDS")]
    max_distance_from_sol: Option<f64>,
    /// Filter the systems that are further than this distance from the reference
    #[clap(long, value_name = "LIGHT_SECONDS", requires = "reference")]
    max_distance_from_reference: Option<f64>,
    /// Filter the systems that have more factions than the number given
    #[clap(long, value_name = "COUNT")]
    max_number_of_factions: Option<usize>,
    /// A reference system used by other filters
    #[clap(
        long,
        value_name = "SYSTEM_NAME",
        requires = "max_distance_from_reference"
    )]
    reference: Option<String>,
    /// Exclude permit locked systems
    #[clap(long, takes_value = false)]
    exclude_permit_locked: bool,
    /// Exclude systems that contain a player faction
    #[clap(long, takes_value = false)]
    exclude_player_faction: bool,
    /// Exclude systems that sell rare commodities
    #[clap(long, takes_value = false)]
    exclude_rare_commodity: bool,
}

pub fn parameters_from_matches<'a, T: System<'a>>(
    matches: &'a Cli,
    systems: &'a [T],
) -> Result<Vec<domain::SystemFilter<'a>>, Error> {
    let reference = matches
        .reference
        .as_ref()
        .map(|reference_name| {
            systems
                .iter()
                .find(|system| system.name() == reference_name)
                .map(domain::System::coordinates)
                .ok_or_else(|| Error::SystemNotFound(reference_name.into()))
        })
        .map_or(Ok(None), |v| v.map(Some))?;

    return Ok(vec![
        matches.allegiance.as_ref().map(|x| allegiance(x)),
        matches.government.as_ref().map(|x| government(x)),
        matches.min_docks_large.map(min_large_docks),
        matches.min_docks.map(min_docks),
        matches.min_starports.map(min_starports),
        matches.max_distance_from_sol.map(max_distance_from_sol),
        matches.min_population.map(min_population),
        matches
            .max_distance_from_reference
            .zip(reference)
            .map(|(distance, reference)| max_distance_from_reference(reference, distance)),
        matches.max_number_of_factions.map(max_number_of_factions),
        if matches.exclude_permit_locked {
            Some(exclude_permit_locked())
        } else {
            None
        },
        if matches.exclude_rare_commodity {
            Some(exclude_rare_commodity())
        } else {
            None
        },
        if matches.exclude_player_faction {
            Some(exclude_player_faction())
        } else {
            None
        },
    ]
    .into_iter()
    .flatten()
    .collect());
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("invalid number: {0:?}")]
    InvalidCount(#[from] ParseIntError),
    #[error("invalid number: {0:?}")]
    InvalidFloat(#[from] ParseFloatError),
    #[error("system not found: {0}")]
    SystemNotFound(String),
}

#[cfg(test)]
mod tests {
    use crate::cli::{parameters_from_matches, Cli};
    use crate::domain::{
        allegiance, government, max_distance_from_reference, max_distance_from_sol, min_docks,
        min_large_docks, min_population, min_starports, Coords,
    };
    use crate::{domain, stub};

    #[test]
    fn no_switches() {
        let args = Cli::default();
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![]
        );
    }

    #[test]
    fn large_docks_present() {
        let args = Cli {
            min_docks_large: Some(10),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_large_docks(10)]
        );
    }

    #[test]
    fn min_population_present() {
        let args = Cli {
            min_population: Some(10),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_population(10)]
        );
    }

    #[test]
    fn docks_present() {
        let args = Cli {
            min_docks: Some(10),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_docks(10)]
        );
    }

    #[test]
    fn starports_present() {
        let args = Cli {
            min_starports: Some(10),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_starports(10)]
        );
    }

    #[test]
    fn population_present() {
        let args = Cli {
            min_population: Some(25_000_000_000),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_population(25_000_000_000)]
        );
    }

    #[test]
    fn allegiance_matches() {
        let args = Cli {
            allegiance: Some("Alliance".to_string()),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![allegiance("Alliance")]
        );
    }

    #[test]
    fn government_matches() {
        let args = Cli {
            government: Some("Democracy".to_string()),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![government("Democracy")]
        );
    }

    #[test]
    fn exclude_permit_locked() {
        let args = Cli {
            exclude_permit_locked: true,
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::exclude_permit_locked()]
        );
    }

    #[test]
    fn exclude_rare_commodity() {
        let args = Cli {
            exclude_rare_commodity: true,
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::exclude_rare_commodity()]
        );
    }

    #[test]
    fn exclude_player_faction() {
        let args = Cli {
            exclude_player_faction: true,
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::exclude_player_faction()]
        );
    }

    #[test]
    fn max_factions_present() {
        let args = Cli {
            max_number_of_factions: Some(10),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::max_number_of_factions(10)]
        );
    }

    #[test]
    fn distance_from_sol_present() {
        let args = Cli {
            max_distance_from_sol: Some(10.),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![max_distance_from_sol(10.0)]
        );
    }

    #[test]
    fn reference_system_not_found() {
        let args = Cli {
            max_distance_from_reference: Some(10.),
            reference: Some("Missing".to_string()),
            ..Cli::default()
        };

        assert!(parameters_from_matches(
            &args,
            &[stub::System {
                name: "Sol".into(),
                coords: Coords {
                    x: f64::from(0),
                    y: f64::from(0),
                    z: f64::from(0),
                },
                stations: vec![],
                population: 0,
                factions: vec![],
                allegiance: String::new(),
                government: String::new()
            }],
        )
        .is_err(),);
    }

    #[test]
    fn both_reference_and_distance_present() {
        let args = Cli {
            max_distance_from_reference: Some(10.),
            reference: Some("Sol".to_string()),
            ..Cli::default()
        };

        assert_eq!(
            parameters_from_matches(
                &args,
                &[stub::System {
                    name: "Sol".into(),
                    coords: Coords {
                        x: f64::from(0),
                        y: f64::from(0),
                        z: f64::from(0),
                    },
                    stations: vec![],
                    population: 0,
                    factions: vec![],
                    allegiance: String::new(),
                    government: String::new(),
                }],
            )
            .unwrap(),
            vec![max_distance_from_reference(
                Coords {
                    x: f64::from(0),
                    y: f64::from(0),
                    z: f64::from(0),
                },
                10.0,
            )]
        );
    }
}
