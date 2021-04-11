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
                .min_starports
                .map_or(true, |starports| has_min_starports(starports, *system))
        })
        .filter(|system| {
            search_options
                .min_population
                .map_or(true, |population| has_min_population(population, *system))
        })
        .filter(|system| {
            if search_options.exclude_permit_locked {
                !is_permit_locked_system(*system)
            } else {
                true
            }
        })
        .filter(|system| {
            if search_options.exclude_rare_commodity {
                !is_exclude_rare_commodity_system(*system)
            } else {
                true
            }
        }))
    .cloned()
    .collect();

    systems.sort_by(|a, b| a.name().cmp(b.name()));
    systems
}

#[allow(clippy::too_many_lines)]
fn is_exclude_rare_commodity_system<T: System>(system: &T) -> bool {
    matches!(
        system.name(),
        "47 Ceti"
            | "Aganippe"
            | "Alacarakmo"
            | "Quechua"
            | "Altair"
            | "Alya"
            | "Anduliga"
            | "Any Na"
            | "Arouca"
            | "AZ Cancri"
            | "Leesti"
            | "BaltahSine"
            | "Banki"
            | "Bast"
            | "Belalans"
            | "Borasetani"
            | "HIP 59533"
            | "CD-75 661"
            | "Alpha Centauri"
            | "Heike"
            | "LDS 883"
            | "Aegaeon"
            | "Cherbones"
            | "Chi Eridani"
            | "Coquim"
            | "Crom"
            | "Bento"
            | "Damna"
            | "Delta Phoenicis"
            | "Deuringas"
            | "Diso"
            | "Aerial"
            | "Eleu"
            | "Eranin"
            | "Eshu"
            | "Esuseku"
            | "Ethgreze"
            | "Fujin"
            | "LHS 3447"
            | "Geawen"
            | "Geras"
            | "Irukama"
            | "Phiagre"
            | "Gilya"
            | "Goman"
            | "Haiden"
            | "Havasupai"
            | "Helvetitj"
            | "HIP 10175"
            | "HIP 118311"
            | "HIP 80364"
            | "HIP 41181"
            | "Holva"
            | "LP 375-25"
            | "HR 7221"
            | "Epsilon Indi"
            | "Colonia"
            | "Jaradharre"
            | "Jaroua"
            | "Jotun"
            | "Kachirigin"
            | "Kamitra"
            | "Kamorin"
            | "Karetii"
            | "Karsuki Ti"
            | "Kinago"
            | "Kongga"
            | "Korro Kung"
            | "Lave"
            | "Zaonce"
            | "Hecate"
            | "LTT 9360"
            | "Tanmark"
            | "Noti"
            | "Mechucos"
            | "Medb"
            | "Mokojing"
            | "Momus Reach"
            | "Dea Motrona"
            | "Mukusubii"
            | "Mulachi"
            | "Neritus"
            | "Ngadandari"
            | "Nguna"
            | "Njangari"
            | "LTT 8517"
            | "Ochoeng"
            | "Kappa Fornacis"
            | "Xelabara"
            | "HIP 112974"
            | "36 Ophiuchi"
            | "Orrere"
            | "George Pantazis"
            | "Delta Pavonis"
            | "Njambalba"
            | "Rajukru"
            | "Rapa Bao"
            | "Rusani"
            | "Sanuma"
            | "Arque"
            | "Ngurii"
            | "Sothis"
            | "Tarach Tor"
            | "Terra Mater"
            | "Thrutis"
            | "Tiolce"
            | "Toxandji"
            | "17 Lyrae"
            | "Uszaa"
            | "Utgaroar"
            | "Uzumoku"
            | "V1090 Herculis"
            | "Vanayequi"
            | "Vega"
            | "Vidavanta"
            | "LFT 1421"
            | "Volkhab"
            | "Shinrarta Dezhra"
            | "Wheemete"
            | "Witchhaul"
            | "Wolf 1301"
            | "Wulpa"
            | "Wuthielo Ku"
            | "Xihe"
            | "Yaso Kondi"
    )
}

fn is_permit_locked_system<T: System>(system: &T) -> bool {
    matches!(
        system.name(),
        "Sol"
            | "Beta Hydri"
            | "Vega"
            | "PLX 695"
            | "Ross 128"
            | "Exbeur"
            | "Hors"
            | "HIP 54530"
            | "4 Sextantis"
            | "CD-44 1695"
            | "HIP 22460"
            | "LFT 509"
            | "Mingfu"
            | "Witch's Reach"
            | "Achenar"
            | "Summerland"
            | "Facece"
            | "Alioth"
            | "Shinrarta Dezhra"
            | "CD-43 11917"
            | "Crom"
            | "Jotun"
            | "Terra Mater"
            | "Sirius"
            | "Isinor"
            | "Hodack"
            | "LTT 198"
            | "Luyten 347-14"
            | "Nastrond"
            | "Peregrina"
            | "Pi Mensae"
            | "Tiliala"
            | "van Maanen's Star"
            | "Alpha Hydri"
            | "Bellica"
            | "Dryio Flyuae IC-B c1-377"
            | "HIP 10332"
            | "HIP 104941"
            | "HIP 22182"
            | "HIP 39425"
            | "HIP 51073"
            | "HIP 87621"
            | "HR 4413"
            | "LHS 2894"
            | "LHS 2921"
            | "LHS 3091"
            | "Mbooni"
            | "Plaa Ain HA-Z d46"
            | "Polaris"
            | "Ross 354"
            | "Scheau Bli NB-O d6-1409"
            | "Wolf 262"
            | "Diso 5 C"
            | "Lave 2"
            | "Moon"
            | "Triton"
            | "Azoth"
            | "Dromi"
            | "Lia Fail"
            | "Matet"
            | "Orna"
            | "Otegine"
            | "Sharur"
            | "Tarnkappe"
            | "Tyet"
            | "Wolfsegen"
    )
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

fn has_min_starports<T: System>(min_starports: usize, system: &T) -> bool {
    system
        .stations()
        .iter()
        .map(|x| x.station_type())
        .filter(|x| {
            matches!(
                *x,
                "Coriolis Starport" | "Ocellus Starport" | "Orbis Starport"
            )
        })
        .count()
        >= min_starports
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
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: false,
                    exclude_rare_commodity: false
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
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: false,
                    exclude_rare_commodity: false
                },
                &input,
            ),
            &[sol]
        )
    }

    #[test]
    fn systems_without_enough_starports_are_skipped() {
        let sol = make_system("Sol", 4, 5, None, Option::from(10000_u128));
        let input = [
            make_system("Sanos", 3, 5, None, Option::from(10000_u128)),
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
                    min_population: None,
                    min_starports: Some(3),
                    exclude_permit_locked: false,
                    exclude_rare_commodity: false
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
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: false,
                    exclude_rare_commodity: false
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
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: false,
                    exclude_rare_commodity: false,
                },
                &input,
            ),
            vec![sol]
        )
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
            ),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: None,
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: true,
                    exclude_rare_commodity: false,
                },
                &input,
            ),
            vec![sanos]
        )
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
            ),
        ];
        assert_eq!(
            filter(
                &SearchOptions {
                    min_large_docks: None,
                    min_docks: None,
                    max_distance_from_sol: None,
                    reference: None,
                    max_distance_from_reference: None,
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: false,
                    exclude_rare_commodity: true,
                },
                &input,
            ),
            vec![sanos]
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
                    min_population: None,
                    min_starports: None,
                    exclude_permit_locked: false,

                    exclude_rare_commodity: false,
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
                    min_population: Option::from(10000_u128),
                    min_starports: None,
                    exclude_permit_locked: false,

                    exclude_rare_commodity: false,
                },
                &input,
            ),
            vec![sol]
        )
    }
}
