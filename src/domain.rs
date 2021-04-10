pub struct Coords {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub trait System {
    fn name(&self) -> &str;
    fn stations(&self) -> Vec<Box<dyn Station>>;
    fn coordinates(&self) -> Coords;
}

pub trait Station {
    fn station_type(&self) -> &str;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SearchOptions {
    pub(crate) min_large_docks: Option<usize>,
    pub(crate) min_docks: Option<usize>,
    pub(crate) max_distance_from_sol: Option<f64>,
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
    ) -> Box<EdsmSystem> {
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
            coords: coordinates.unwrap_or(EdsmCoords {
                x: 73.875_f64,
                y: -3.5625_f64,
                z: -52.625_f64,
            }),
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
        let input = vec![
            make_system("Sanos", 5, 5, None),
            make_system("Sol", 5, 5, None),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: None,
                },
                input.clone(),
            ),
            input
        )
    }

    #[test]
    fn systems_without_enough_large_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5, None);
        let input = vec![make_system("Sanos", 2, 5, None), sol.clone()];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: Some(5),
                    min_docks: None,
                    max_distance_from_sol: None,
                },
                input,
            ),
            vec![sol]
        )
    }

    #[test]
    fn systems_without_enough_docks_are_skipped() {
        let sol = make_system("Sol", 5, 5, None);
        let input = vec![make_system("Sanos", 2, 5, None), sol.clone()];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: Some(9),
                    max_distance_from_sol: None,
                },
                input,
            ),
            vec![sol]
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
        );
        let input = vec![
            make_system(
                "Sanos",
                2,
                5,
                Option::from(EdsmCoords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                }),
            ),
            sol.clone(),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: Some(90.0),
                },
                input,
            ),
            vec![sol]
        )
    }
}
