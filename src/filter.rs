use crate::distance;
use crate::domain::{Coords, System, SystemFilter};
use std::collections::HashSet;

pub fn filter<'a, T: System + Clone>(
    search_options: &'a [SystemFilter],
    systems: &'a [T],
) -> Vec<T> {
    let mut systems: Vec<_> = systems
        .iter()
        .filter(|system| {
            search_options.iter().all(|filter| match filter {
                SystemFilter::MaximumDistanceFrom(reference, distance_from_reference_ls) => {
                    has_location_within_max_distance_from_reference(
                        *distance_from_reference_ls,
                        *reference,
                        *system,
                    )
                }
                SystemFilter::MinimumStationCount(types, docks) => {
                    has_docks(*docks, types, *system)
                }
                SystemFilter::MaximumFactionCount(factions) => {
                    has_max_number_of_factions(*factions, *system)
                }
                SystemFilter::MinimumPopulation(population) => {
                    has_min_population(*population, *system)
                }
                SystemFilter::ExcludeSystems(systems) => !is_excluded_system(systems, *system),
                SystemFilter::ExcludeSystemsWithPlayerFactions => !has_player_faction(*system),
            })
        })
        .cloned()
        .collect();

    systems.sort_by(|a, b| a.name().cmp(b.name()));
    systems
}

fn is_excluded_system<T: System>(excluded_systems: &HashSet<String>, system: &T) -> bool {
    excluded_systems.contains(system.name())
}

fn has_docks<T: System>(min_large_docks: usize, types: &HashSet<String>, system: &T) -> bool {
    system
        .stations()
        .iter()
        .map(|x| x.station_type())
        .filter(|x| types.contains(*x))
        .count()
        >= min_large_docks
}

fn has_max_number_of_factions<T: System>(max_factions: usize, system: &T) -> bool {
    system.factions().len() <= max_factions
}

fn has_player_faction<T: System>(system: &T) -> bool {
    system.factions().iter().any(|faction| faction.is_player())
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

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::domain::{
        exclude_permit_locked, exclude_player_faction, exclude_rare_commodity,
        max_distance_from_reference, max_distance_from_sol, max_number_of_factions, min_docks,
        min_large_docks, min_population, min_starports,
    };
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
        player_factions: bool,
        faction_count: usize,
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
            station_type: String::from("N/A"),
            distance_to_arrival: Some(296.864_456),
        })
        .map(|_x| EdsmStation {
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
            factions: Some(
                vec![EdsmFaction {
                    is_player: player_factions,
                }]
                .into_iter()
                .cycle()
                .take(faction_count)
                .collect(),
            ),
            stations: Some(stations.collect()),
        }
    }

    #[test]
    fn no_options_returns_everything() {
        let input = [
            make_system("Sanos", 5, 5, None, Option::from(10000_u128), true, 5),
            make_system("Sol", 5, 5, None, Option::from(10000_u128), true, 5),
        ];
        assert_eq!(filter(&[], &input,), input)
    }

    #[test]
    fn systems_without_enough_large_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5, None, Option::from(10000_u128), true, 5);
        let input = [
            make_system("Sanos", 2, 5, None, Option::from(10000_u128), true, 5),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_large_docks(5)], &input,), &[sol])
    }

    #[test]
    fn systems_without_enough_starports_are_skipped() {
        let sol = make_system("Sol", 4, 5, None, Option::from(10000_u128), true, 5);
        let input = [
            make_system("Sanos", 3, 5, None, Option::from(10000_u128), true, 5),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_starports(3)], &input,), &[sol])
    }

    #[test]
    fn systems_without_enough_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5, None, Option::from(10000_u128), true, 5);
        let input = [
            make_system("Sanos", 2, 5, None, Option::from(10000_u128), true, 5),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_docks(9)], &input,), &[sol])
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
            true,
            5,
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
                true,
                5,
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[max_distance_from_sol(90.0)], &input,), vec![sol])
    }

    #[test]
    fn permit_locked_systems_skipped() {
        let sanos = make_system(
            "Sanos",
            2,
            5,
            Option::from(EdsmCoords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            }),
            Option::from(10000_u128),
            true,
            5,
        );
        let input = [
            sanos.clone(),
            make_system(
                "Sol",
                5,
                5,
                Option::from(EdsmCoords {
                    x: f64::from(0),
                    y: f64::from(0),
                    z: f64::from(0),
                }),
                Option::from(10000_u128),
                true,
                5,
            ),
        ];
        assert_eq!(filter(&[exclude_permit_locked()], &input,), vec![sanos])
    }

    #[test]
    fn rare_commodity_systems_skipped() {
        let sanos = make_system(
            "Sanos",
            2,
            5,
            Option::from(EdsmCoords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            }),
            Option::from(10000_u128),
            true,
            5,
        );
        let input = [
            sanos.clone(),
            make_system(
                "Alpha Centauri",
                5,
                5,
                Option::from(EdsmCoords {
                    x: f64::from(0),
                    y: f64::from(0),
                    z: f64::from(0),
                }),
                Option::from(10000_u128),
                true,
                5,
            ),
        ];
        assert_eq!(filter(&[exclude_rare_commodity()], &input,), vec![sanos])
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
            true,
            5,
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
                true,
                5,
            ),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &[max_distance_from_reference(
                    crate::domain::Coords {
                        x: f64::from(0),
                        y: f64::from(0),
                        z: f64::from(0),
                    },
                    90.0
                )],
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
            true,
            5,
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
                true,
                5,
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_population(10000_u128)], &input,), vec![sol])
    }

    #[test]
    fn systems_with_too_many_factions_are_ignored_ignored() {
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
            false,
            5,
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
                true,
                10,
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[max_number_of_factions(7)], &input,), vec![sol])
    }

    #[test]
    fn systems_with_player_factions_are_ignored_ignored() {
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
            false,
            5,
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
                true,
                10,
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[exclude_player_faction()], &input,), vec![sol])
    }
}
