pub trait System {
    fn name(&self) -> &str;
    fn stations(&self) -> Vec<Box<dyn Station>>;
}

pub trait Station {
    fn station_type(&self) -> &str;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SearchOptions {
    pub(crate) min_large_docks: Option<usize>,
    pub(crate) min_docks: Option<usize>,
}

pub fn filter<T: System>(search_options: &SearchOptions, systems: Vec<Box<T>>) -> Vec<Box<T>> {
    (systems
        .into_iter()
        .filter(|system| {
            search_options
                .min_docks
                .map_or(true, |docks| has_min_docks(docks, system.as_ref()))
        })
        .filter(|system| {
            search_options
                .min_large_docks
                .map_or(true, |docks| has_min_large_docks(docks, system.as_ref()))
        }))
    .collect()
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

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::domain::{filter, SearchOptions};
    use crate::edsm::{
        ControllingFaction as EdsmControllingFaction, Coords as EdsmCoords, Faction as EdsmFaction,
        Station as EdsmStation, System as EdsmSystem,
    };

    fn make_system(name: &str, large_docks: usize, small_docks: usize) -> Box<EdsmSystem> {
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

        Box::from(EdsmSystem {
            name: String::from(name),
            coords: EdsmCoords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            },
            population: Some(6_149_044_545),
            controlling_faction: EdsmControllingFaction {
                allegiance: Some("Federation".to_string()),
            },
            factions: Some(vec![
                EdsmFaction { is_player: true },
                EdsmFaction { is_player: false },
            ]),
            stations: Some(stations.collect()),
        })
    }

    #[test]
    fn no_options_returns_everything() {
        let input = vec![make_system("Sanos", 5, 5), make_system("Sol", 5, 5)];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                },
                input.clone(),
            ),
            input
        )
    }

    #[test]
    fn systems_without_enough_large_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5);
        let input = vec![make_system("Sanos", 2, 5), sol.clone()];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: Some(5),
                    min_docks: None,
                },
                input,
            ),
            vec![sol]
        )
    }

    #[test]
    fn systems_without_enough_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5);
        let input = vec![make_system("Sanos", 2, 5), sol.clone()];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: Some(9),
                },
                input,
            ),
            vec![sol]
        )
    }
}
