use crate::distance;
use crate::domain::{Coords, SearchOptions, System};

pub fn filter<'a, T: System + Clone>(
    search_options: &'a SearchOptions,
    systems: &'a [T],
) -> Vec<T> {
    let mut systems: Vec<_> = (systems
        .iter()
        .filter(|system| {
            search_options
                .max_distance_from_sol
                .map_or(true, |distance_from_sol_ls| {
                    has_location_within_max_distance_from_sol(distance_from_sol_ls, *system)
                })
        })
        .filter(|system| {
            search_options
                .reference
                .and_then(|x| search_options.max_distance_from_reference.map(|y| (x, y)))
                .map_or(true, |(reference, distance_from_reference_ls)| {
                    has_location_within_max_distance_from_reference(
                        distance_from_reference_ls,
                        reference,
                        *system,
                    )
                })
        })
        .filter(|system| {
            search_options
                .min_docks
                .map_or(true, |docks| has_min_docks(docks, *system))
        })
        .filter(|system| {
            search_options
                .min_large_docks
                .map_or(true, |docks| has_min_large_docks(docks, *system))
        })
        .filter(|system| {
            search_options
                .min_population
                .map_or(true, |population| has_min_population(population, *system))
        }))
    .cloned()
    .collect();

    systems.sort_by(|a, b| a.name().cmp(b.name()));
    systems
}

fn has_min_large_docks<T: System>(min_large_docks: usize, system: &T) -> bool {
    system
        .stations()
        .iter()
        .map(|x| x.station_type())
        .filter(|x| {
            matches!(
                *x,
                "Asteroid base" | "Coriolis Starport" | "Ocellus Starport" | "Orbis Starport"
            )
        })
        .count()
        >= min_large_docks
}

fn has_min_docks<T: System>(min_large_docks: usize, system: &T) -> bool {
    system
        .stations()
        .iter()
        .map(|x| x.station_type())
        .filter(|x| {
            matches!(
                *x,
                "Asteroid base"
                    | "Coriolis Starport"
                    | "Ocellus Starport"
                    | "Orbis Starport"
                    | "Outpost"
                    | "Planetary Outpost"
                    | "Planetary Port"
            )
        })
        .count()
        >= min_large_docks
}

fn has_min_population<T: System>(min_population: u128, system: &T) -> bool {
    system.population() >= min_population
}

fn has_location_within_max_distance_from_reference<T: System>(
    distance_from_reference_ls: f64,
    reference: Coords,
    system: &T,
) -> bool {
    distance::distance(&reference, &system.coordinates()) <= distance_from_reference_ls as f64
}

fn has_location_within_max_distance_from_sol<T: System>(
    distance_from_sol_ls: f64,
    system: &T,
) -> bool {
    distance::distance(
        &Coords {
            x: f64::from(0),
            y: f64::from(0),
            z: f64::from(0),
        },
        &system.coordinates(),
    ) <= distance_from_sol_ls as f64
}

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::domain::SearchOptions;
    use crate::edsm::{
        ControllingFaction as EdsmControllingFaction, Coords as EdsmCoords, Faction as EdsmFaction,
        Station as EdsmStation, System as EdsmSystem,
    };
    use crate::filter::filter;

    fn make_system(
        name: &str,
        large_docks: usize,
        small_docks: usize,
        coordinates: Option<EdsmCoords>,
        population: Option<u128>,
    ) -> EdsmSystem {
        let mut large_station_types = [
            "Asteroid base",
            "Coriolis Starport",
            "Ocellus Starport",
            "Orbis Starport",
        ]
        .iter()
        .cycle();

        let stations = iter::repeat(EdsmStation {
            station_type: String::from(*large_station_types.next().unwrap()),
            distance_to_arrival: Some(296.864_456),
        })
        .take(large_docks)
        .chain(
            iter::repeat(EdsmStation {
                station_type: "Planetary Outpost".to_string(),
                distance_to_arrival: Some(296.864_456),
            })
            .take(small_docks),
        );

        EdsmSystem {
            name: String::from(name),
            coords: coordinates.unwrap_or(EdsmCoords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            }),
            population,
            controlling_faction: EdsmControllingFaction {
                allegiance: Some("Federation".to_string()),
            },
            factions: Some(vec![
                EdsmFaction { is_player: true },
                EdsmFaction { is_player: false },
            ]),
            stations: Some(stations.collect()),
        }
    }

    #[test]
    fn no_options_returns_everything() {
        let input = [
            make_system("Sanos", 5, 5, None, Option::from(10000_u128)),
            make_system("Sol", 5, 5, None, Option::from(10000_u128)),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: None,
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: None
                },
                &input,
            ),
            input
        )
    }

    #[test]
    fn systems_without_enough_large_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5, None, Option::from(10000_u128));
        let input = [
            make_system("Sanos", 2, 5, None, Option::from(10000_u128)),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: Some(5),
                    min_docks: None,
                    max_distance_from_sol: None,
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: None
                },
                &input,
            ),
            &[sol]
        )
    }

    #[test]
    fn systems_without_enough_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5, None, Option::from(10000_u128));
        let input = [
            make_system("Sanos", 2, 5, None, Option::from(10000_u128)),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: Some(9),
                    max_distance_from_sol: None,
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: None
                },
                &input,
            ),
            &[sol]
        )
    }

    #[test]
    fn systems_too_far_from_sol_skipped() {
        let sol = make_system(
            "Sol",
            5,
            5,
            Option::from(EdsmCoords {
                x: f64::from(0),
                y: f64::from(0),
                z: f64::from(0),
            }),
            Option::from(10000_u128),
        );
        let input = [
            make_system(
                "Sanos",
                2,
                5,
                Option::from(EdsmCoords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                }),
                Option::from(10000_u128),
            ),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: Some(90.0),
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: None
                },
                &input,
            ),
            vec![sol]
        )
    }

    #[test]
    fn systems_too_far_from_reference_skipped() {
        let sol = make_system(
            "Sol",
            5,
            5,
            Option::from(EdsmCoords {
                x: f64::from(0),
                y: f64::from(0),
                z: f64::from(0),
            }),
            Option::from(10000_u128),
        );
        let input = [
            make_system(
                "Sanos",
                2,
                5,
                Option::from(EdsmCoords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                }),
                Option::from(10000_u128),
            ),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: None,
                    reference: Option::from(crate::domain::Coords {
                        x: f64::from(0),
                        y: f64::from(0),
                        z: f64::from(0),
                    }),
                    max_distance_from_reference: Some(90.0),
                    min_population: None
                },
                &input,
            ),
            vec![sol]
        )
    }

    #[test]
    fn systems_with_too_low_population_are_ignored() {
        let sol = make_system(
            "Sol",
            5,
            5,
            Option::from(EdsmCoords {
                x: f64::from(0),
                y: f64::from(0),
                z: f64::from(0),
            }),
            Option::from(10000_u128),
        );
        let input = [
            make_system(
                "Sanos",
                2,
                5,
                Option::from(EdsmCoords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                }),
                Option::from(9999_u128),
            ),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: None,
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: Option::from(10000_u128)
                },
                &input,
            ),
            vec![sol]
        )
    }
}
