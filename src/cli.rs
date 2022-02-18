use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use clap::{crate_authors, crate_version, Arg, ArgMatches, Command};
use thiserror::Error as ThisError;

use crate::domain;
use crate::domain::{
    allegiance, exclude_permit_locked, exclude_player_faction, exclude_rare_commodity, government,
    max_distance_from_reference, max_distance_from_sol, max_number_of_factions, min_docks,
    min_large_docks, min_population, min_starports, System,
};

#[allow(clippy::too_many_lines)]
pub fn cli() -> Command<'static> {
    Command::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("edsm-path")
                .help("This is the path to th EDSM dump in .json.gz format")
                .required(true),
        )
        .arg(
            Arg::new("min-docks-large")
                .help(
                    "Filter the systems that are have less than the given number of docks with room for large ships",
                )
                .long("min-docks-large")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
        .arg(
            Arg::new("allegiance")
                .help(
                    "Filter by allegiance",
                )
                .long("allegiance")
                .takes_value(true)
                .value_name("MAJOR_FACTION")
                .required(false),
        )
        .arg(
            Arg::new("government")
                .help(
                    "Filter by government",
                )
                .long("government")
                .takes_value(true)
                .value_name("GOVERNMENT_TYPE")
                .required(false),
        )
        .arg(
            Arg::new("min-starports")
                .help(
                    "Filter the systems that are have less than the given number of starports"
                )
                .long("min-starports")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
        .arg(
            Arg::new("min-docks")
                .help(
                    "Filter the systems that are have less than the given number of docks"
                )
                .long("min-docks")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
        .arg(
            Arg::new("min-population")
                .help(
                    "Filter the systems that are have less than the given population"
                )
                .long("min-population")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
        .arg(
            Arg::new("max-distance-from-sol")
                .help(
                    "Filter the systems that are further than this distance from sol"
                )
                .long("max-distance-from-sol")
                .takes_value(true)
                .value_name("LIGHT_SECONDS")
                .required(false),
        )
        .arg(
            Arg::new("max-distance-from-reference")
                .help(
                    "Filter the systems that are further than this distance from the reference"
                )
                .long("max-distance-from-reference")
                .takes_value(true)
                .value_name("LIGHT_SECONDS")
                .required(false),
        )
        .arg(
            Arg::new("max-number-of-factions")
                .help(
                    "Filter the systems that have more factions than the number given"
                )
                .long("max-number-of-factions")
                .takes_value(true)
                .value_name("COUNT")
                .required(false),
        )
        .arg(
            Arg::new("reference")
                .help(
                    "A reference system used by other filters"
                )
                .long("reference")
                .takes_value(true)
                .value_name("SYSTEM_NAME")
                .required(false),
        )
        .arg(
            Arg::new("exclude-permit-locked")
                .help(
                    "Exclude permit locked systems"
                )
                .long("exclude-permit-locked")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("exclude-player-faction")
                .help(
                    "Exclude systems that contain a player faction"
                )
                .long("exclude-player-faction")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("exclude-rare-commodity")
                .help(
                    "Exclude systems that sell rare commodities"
                )
                .long("exclude-rare-commodity")
                .takes_value(false)
                .required(false),
        )
}

pub fn parameters_from_matches<'a, T: System<'a>>(
    matches: &'a ArgMatches,
    systems: &'a [T],
) -> Result<Vec<domain::SystemFilter<'a>>, Error> {
    let reference = matches
        .value_of("reference")
        .map(|reference_name| {
            systems
                .iter()
                .find(|system| system.name() == reference_name)
                .map(domain::System::coordinates)
                .ok_or_else(|| Error::SystemNotFound(reference_name.into()))
        })
        .map_or(Ok(None), |v| v.map(Some))?;

    return Ok(vec![
        matches.value_of("allegiance").map(allegiance),
        matches.value_of("government").map(government),
        matches
            .value_of("min-docks-large")
            .map(|value| usize::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(|x| Some(min_large_docks(x))))?,
        matches
            .value_of("min-docks")
            .map(|value| usize::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(|x| Some(min_docks(x))))?,
        matches
            .value_of("min-starports")
            .map(|value| usize::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(|x| Some(min_starports(x))))?,
        matches
            .value_of("max-distance-from-sol")
            .map(|value| f64::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(|x| Some(max_distance_from_sol(x))))?,
        matches
            .value_of("min-population")
            .map(|value| u128::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(|x| Some(min_population(x))))?,
        matches
            .value_of("max-distance-from-reference")
            .map(|value| f64::from_str(value).map_err(Error::from))
            .and_then(|value| reference.map(|reference| value.map(|x| (reference, x))))
            .map_or(Ok(None), |v| {
                v.map(|(reference, value)| Some(max_distance_from_reference(reference, value)))
            })?,
        matches
            .value_of("max-number-of-factions")
            .map(|value| usize::from_str(value).map_err(Error::from))
            .map_or(Ok(None), |v| v.map(|x| Some(max_number_of_factions(x))))?,
        if matches.is_present("exclude-permit-locked") {
            Some(exclude_permit_locked())
        } else {
            None
        },
        if matches.is_present("exclude-rare-commodity") {
            Some(exclude_rare_commodity())
        } else {
            None
        },
        if matches.is_present("exclude-player-faction") {
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
    use crate::cli::{cli, parameters_from_matches};
    use crate::domain::{
        allegiance, government, max_distance_from_reference, max_distance_from_sol, min_docks,
        min_large_docks, min_population, min_starports, Coords,
    };
    use crate::{domain, stub};

    #[test]
    fn no_switches() {
        let args = cli().get_matches_from(vec!["ed-system-search", "some-edsm-dump.json.gz"]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![]
        );
    }

    #[test]
    fn large_docks_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-docks-large=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err());
    }

    #[test]
    fn large_docks_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-docks-large=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_large_docks(10)]
        );
    }

    #[test]
    fn min_population_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-population=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err(),);
    }

    #[test]
    fn min_population_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-population=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_population(10)]
        );
    }

    #[test]
    fn docks_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-docks=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err(),);
    }

    #[test]
    fn docks_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-docks=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_docks(10)]
        );
    }

    #[test]
    fn starports_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-starports=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err(),);
    }

    #[test]
    fn starports_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-starports=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_starports(10)]
        );
    }

    #[test]
    fn population_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-population=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err(),);
    }

    #[test]
    fn population_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--min-population=25000000000",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![min_population(25_000_000_000)]
        );
    }

    #[test]
    fn allegiance_matches() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--allegiance=Alliance",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![allegiance("Alliance")]
        );
    }

    #[test]
    fn government_matches() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--government=Democracy",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![government("Democracy")]
        );
    }

    #[test]
    fn exclude_permit_locked() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--exclude-permit-locked",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::exclude_permit_locked()]
        );
    }

    #[test]
    fn exclude_rare_commodity() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--exclude-rare-commodity",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::exclude_rare_commodity()]
        );
    }

    #[test]
    fn exclude_player_faction() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--exclude-player-faction",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::exclude_player_faction()]
        );
    }

    #[test]
    fn max_factions_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-number-of-factions=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err(),);
    }

    #[test]
    fn max_factions_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-number-of-factions=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![domain::max_number_of_factions(10)]
        );
    }

    #[test]
    fn distance_from_sol_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-distance-from-sol=banana",
            "some-edsm-dump.json.gz",
        ]);
        assert!(parameters_from_matches(&args, &[] as &[stub::System]).is_err(),);
    }

    #[test]
    fn distance_from_sol_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-distance-from-sol=10",
            "some-edsm-dump.json.gz",
        ]);
        assert_eq!(
            parameters_from_matches(&args, &[] as &[stub::System]).unwrap(),
            vec![max_distance_from_sol(10.0)]
        );
    }

    #[test]
    fn distance_from_reference_invalid() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-distance-from-reference=banana",
            "--reference=Sol",
            "some-edsm-dump.json.gz",
        ]);
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
                allegiance: "".to_string(),
                government: "".to_string()
            }],
        )
        .is_err(),);
    }

    #[test]
    fn reference_system_not_found() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-distance-from-reference=10",
            "--reference=Missing",
            "some-edsm-dump.json.gz",
        ]);
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
                allegiance: "".to_string(),
                government: "".to_string()
            }],
        )
        .is_err(),);
    }

    #[test]
    fn both_reference_and_distance_present() {
        let args = cli().get_matches_from(vec![
            "ed-system-search",
            "--max-distance-from-reference=10",
            "--reference=Sol",
            "some-edsm-dump.json.gz",
        ]);
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
                    allegiance: "".to_string(),
                    government: "".to_string()
                }],
            )
            .unwrap(),
            vec![max_distance_from_reference(
                Coords {
                    x: f64::from(0),
                    y: f64::from(0),
                    z: f64::from(0),
                },
                10.0
            )]
        );
    }
}
