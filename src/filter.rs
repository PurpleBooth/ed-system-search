use std::collections::HashSet;

use crate::distance;
use crate::domain::{Coords, System, SystemFilter};

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
                SystemFilter::Allegiance(allegiance) => has_allegiance(allegiance, *system),
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
fn has_allegiance<T: System>(allegiance: &str, system: &T) -> bool {
    system.allegiance().eq(allegiance)
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

    use crate::domain::{
        allegiance, exclude_permit_locked, exclude_player_faction, exclude_rare_commodity,
        max_distance_from_reference, max_distance_from_sol, max_number_of_factions, min_docks,
        min_large_docks, min_population, min_starports,
    };

    use crate::filter::filter;
    use crate::stub::Faction;
    use crate::{domain, stub};

    fn make_system(name: &str) -> stub::System {
        stub::System {
            name: String::from(name),
            coords: domain::Coords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            },
            population: 0,
            factions: vec![],
            stations: vec![],
            allegiance: "".to_string(),
        }
    }

    fn make_system_with_allegiance(name: &str, allegiance: &str) -> stub::System {
        stub::System {
            name: String::from(name),
            coords: domain::Coords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            },
            allegiance: String::from(allegiance),
            population: 0,
            factions: vec![],
            stations: vec![],
        }
    }

    fn make_system_with_population(name: &str, population: u128) -> stub::System {
        stub::System {
            name: String::from(name),
            coords: domain::Coords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            },
            population,
            factions: vec![],
            stations: vec![],
            allegiance: "".to_string(),
        }
    }

    fn make_stub_system_with_docks(name: &str, docks: &[&str]) -> stub::System {
        stub::System {
            name: String::from(name),
            coords: domain::Coords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            },
            population: 1,
            factions: vec![],
            stations: docks
                .iter()
                .map(|x| stub::Station {
                    station_type: String::from(*x),
                })
                .collect(),
            allegiance: "".to_string(),
        }
    }

    fn make_system_with_factions(name: &str, factions: &[bool]) -> stub::System {
        stub::System {
            name: String::from(name),
            coords: domain::Coords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            },
            population: 1,
            factions: factions
                .iter()
                .map(|player| Faction { is_player: *player })
                .collect(),
            stations: vec![],
            allegiance: "".to_string(),
        }
    }

    fn make_system_at_coords(name: &str, coords: domain::Coords) -> stub::System {
        stub::System {
            name: String::from(name),
            coords,
            population: 1,
            factions: vec![],
            stations: vec![],
            allegiance: "".to_string(),
        }
    }

    #[test]
    fn no_options_returns_everything() {
        let input = [make_system("Sanos"), make_system("Sol")];
        assert_eq!(filter(&[], &input), input)
    }

    #[test]
    fn systems_without_enough_large_docks_are_skipped() {
        let sol = make_stub_system_with_docks(
            "Sol",
            &[
                "Asteroid base",
                "Coriolis Starport",
                "Ocellus Starport",
                "Orbis Starport",
                "Planetary Outpost",
                "Ocellus Starport",
            ],
        );
        let input = [
            make_stub_system_with_docks(
                "Sanos",
                &[
                    "Asteroid base",
                    "Ocellus Starport",
                    "Orbis Starport",
                    "Planetary Outpost",
                ],
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_large_docks(5)], &input,), &[sol])
    }

    #[test]
    fn systems_without_enough_starports_are_skipped() {
        let sol = make_stub_system_with_docks(
            "Sol",
            &["Coriolis Starport", "Ocellus Starport", "Orbis Starport"],
        );
        let input = [
            make_stub_system_with_docks(
                "Sanos",
                &["Asteroid base", "Orbis Starport", "Planetary Outpost"],
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_starports(3)], &input,), &[sol])
    }

    #[test]
    fn systems_without_enough_docks_are_skipped() {
        let sol = make_stub_system_with_docks("Sol", &["Asteroid base", "Planetary Outpost"]);
        let input = [
            make_stub_system_with_docks("Sanos", &["Planetary Outpost"]),
            sol.clone(),
        ];
        assert_eq!(filter(&[min_docks(2)], &input,), &[sol])
    }

    #[test]
    fn systems_too_far_from_sol_skipped() {
        let sol = make_system_at_coords(
            "Sol",
            domain::Coords {
                x: f64::from(0),
                y: f64::from(0),
                z: f64::from(0),
            },
        );

        let input = [
            make_system_at_coords(
                "Sanos",
                domain::Coords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                },
            ),
            sol.clone(),
        ];
        assert_eq!(filter(&[max_distance_from_sol(90.0)], &input), vec![sol])
    }

    #[test]
    fn permit_locked_systems_skipped() {
        let sanos = make_system("Sanos");
        let input = [sanos.clone(), make_system("Sol")];
        assert_eq!(filter(&[exclude_permit_locked()], &input), vec![sanos])
    }

    #[test]
    fn rare_commodity_systems_skipped() {
        let sanos = make_system("Sanos");
        let input = [sanos.clone(), make_system("Alpha Centauri")];
        assert_eq!(filter(&[exclude_rare_commodity()], &input), vec![sanos])
    }

    #[test]
    fn systems_too_far_from_reference_skipped() {
        let sol = make_system_at_coords(
            "Sol",
            domain::Coords {
                x: f64::from(0),
                y: f64::from(0),
                z: f64::from(0),
            },
        );
        let input = [
            make_system_at_coords(
                "Sanos",
                domain::Coords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                },
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
                    90.0,
                )],
                &input,
            ),
            vec![sol]
        )
    }

    #[test]
    fn systems_with_too_low_population_are_ignored() {
        let sol = make_system_with_population("Sol", 10000_u128);
        let input = [make_system_with_population("Sanos", 9999_u128), sol.clone()];
        assert_eq!(filter(&[min_population(10000_u128)], &input), vec![sol])
    }

    #[test]
    fn systems_with_too_many_factions_are_ignored_ignored() {
        let sol = make_system_with_factions("Sol", &[false, false, false]);
        let input = [
            make_system_with_factions("Sanos", &[false, false, false, false]),
            sol.clone(),
        ];
        assert_eq!(filter(&[max_number_of_factions(3)], &input), vec![sol])
    }

    #[test]
    fn systems_with_player_factions_are_ignored_ignored() {
        let sol = make_system_with_factions("Sol", &[false, false]);
        let input = [
            make_system_with_factions("Sanos", &[false, true]),
            sol.clone(),
        ];
        assert_eq!(filter(&[exclude_player_faction()], &input), vec![sol])
    }

    #[test]
    fn systems_allegiance() {
        let sol = make_system_with_allegiance("Sol", "Alliance");
        let input = [
            make_system_with_allegiance("Sanos", "Federation"),
            sol.clone(),
        ];
        assert_eq!(
            filter(&[allegiance(String::from("Alliance"))], &input),
            vec![sol]
        )
    }
}
