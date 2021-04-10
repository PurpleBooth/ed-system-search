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
    use indoc::indoc;

    use crate::domain::System as DomainSystem;
    use crate::edsm::{parse, ControllingFaction, Coords, Faction, Station, System};

    #[test]
    fn invalid_json() {
        assert_eq!(parse("This is not valid json".as_bytes()).is_err(), true)
    }

    #[test]
    fn valid_json() {
        let example_json = indoc! {r#"
            [{"id":8624,"id64":3932277445322,"name":"Sanos","coords":{"x":73.875,"y":-3.5625,"z":-52.625},"allegiance":"Federation","government":"Corporate","state":"None","economy":"Agriculture","security":"High","population":6149044545,"controllingFaction":{"id":2382,"name":"Federal Liberal Command","allegiance":"Federation","government":"Corporate","isPlayer":true},"factions":[{"id":2382,"name":"Federal Liberal Command","allegiance":"Federation","government":"Corporate","influence":0.523904,"state":"None","activeStates":[],"recoveringStates":[],"pendingStates":[],"happiness":"Happy","isPlayer":true,"lastUpdate":1616625843},{"id":9276,"name":"Independents of Sanos","allegiance":"Federation","government":"Democracy","influence":0.167331,"state":"None","activeStates":[],"recoveringStates":[],"pendingStates":[],"happiness":"Happy","isPlayer":false,"lastUpdate":1616625843}],"stations":[{"id":79936,"marketId":3705068032,"type":"Fleet Carrier","name":"K9T-38W","distanceToArrival":296.807847,"allegiance":"Independent","government":"Fleet Carrier","economy":"Fleet Carrier","secondEconomy":null,"haveMarket":true,"haveShipyard":false,"haveOutfitting":false,"otherServices":["Black Market","Restock","Refuel","Repair","Contacts","Universal Cartographics","Crew Lounge"],"updateTime":{"information":"2021-03-24 21:49:02","market":"2021-03-24 21:49:10","shipyard":null,"outfitting":null}},{"id":23320,"marketId":3228284928,"type":"Orbis Starport","name":"Apgar Terminal","distanceToArrival":296.864456,"allegiance":"Federation","government":"Democracy","economy":"Terraforming","secondEconomy":null,"haveMarket":true,"haveShipyard":true,"haveOutfitting":true,"otherServices":["Black Market","Restock","Refuel","Repair","Contacts","Universal Cartographics","Missions","Crew Lounge","Tuning","Search and Rescue"],"controllingFaction":{"id":9276,"name":"Independents of Sanos"},"updateTime":{"information":"2021-03-24 21:57:14","market":"2021-03-24 21:57:19","shipyard":"2021-03-13 21:01:03","outfitting":"2021-03-24 21:57:19"}}],"bodies":[{"id":9905400,"id64":36032729296409290,"bodyId":1,"name":"Sanos A","type":"Star","subType":"K (Yellow-Orange) Star","parents":[{"Null":0}],"distanceToArrival":0,"isMainStar":true,"isScoopable":true,"age":12692,"spectralClass":"K7","luminosity":"Va","absoluteMagnitude":6.602951,"solarMasses":0.683594,"solarRadius":0.8590392868439971,"surfaceTemperature":4145,"orbitalPeriod":149157.4087628588,"semiMajorAxis":42.891440322239156,"orbitalEccentricity":0.057784,"orbitalInclination":10.679358,"argOfPeriapsis":167.826234,"rotationalPeriod":3.4478455151967595,"rotationalPeriodTidallyLocked":false,"axialTilt":null,"updateTime":"2021-02-23 11:31:41"},{"id":9905405,"id64":216176714391229120,"bodyId":6,"name":"Sanos A 1","type":"Planet","subType":"Earth-like world","parents":[{"Null":5},{"Star":1},{"Null":0}],"distanceToArrival":297,"isLandable":false,"gravity":0.9482140560624709,"earthMasses":0.691103,"radius":5445.0605,"surfaceTemperature":276,"surfacePressure":2.3014628053293857,"volcanismType":"No volcanism","atmosphereType":"Suitable for water-based life","atmosphereComposition":{"Nitrogen":91.25,"Oxygen":8.69,"Carbon dioxide":0.06},"solidComposition":{"Rock":67.21,"Metal":32.79,"Ice":0},"terraformingState":"Terraformed","orbitalPeriod":15.02531201199074,"semiMajorAxis":0.0007772675439048879,"orbitalEccentricity":0.105683,"orbitalInclination":0.412458,"argOfPeriapsis":182.25922,"rotationalPeriod":23.04342612039352,"rotationalPeriodTidallyLocked":false,"axialTilt":-0.163319,"updateTime":"2021-03-14 03:49:33"}],"date":"2017-02-23 01:23:25"}]
        "#};

        assert_eq!(
            parse(example_json.as_bytes()).unwrap(),
            vec![System {
                name: String::from("Sanos"),
                coords: Coords {
                    x: 73.875_f64,
                    y: -3.5625_f64,
                    z: -52.625_f64,
                },
                population: Some(6_149_044_545),
                controlling_faction: ControllingFaction {
                    allegiance: Some("Federation".to_string())
                },
                factions: Some(vec![
                    Faction { is_player: true },
                    Faction { is_player: false }
                ]),
                stations: Some(vec![
                    Station {
                        station_type: String::from("Fleet Carrier"),
                        distance_to_arrival: Some(296.807_847),
                    },
                    Station {
                        station_type: "Orbis Starport".to_string(),
                        distance_to_arrival: Some(296.864_456),
                    }
                ]),
            }]
        )
    }

    #[test]
    fn implements_system_trait() {
        let example_json = indoc! {r#"
            [{"id":8624,"id64":3932277445322,"name":"Sanos","coords":{"x":73.875,"y":-3.5625,"z":-52.625},"allegiance":"Federation","government":"Corporate","state":"None","economy":"Agriculture","security":"High","population":6149044545,"controllingFaction":{"id":2382,"name":"Federal Liberal Command","allegiance":"Federation","government":"Corporate","isPlayer":true},"factions":[{"id":2382,"name":"Federal Liberal Command","allegiance":"Federation","government":"Corporate","influence":0.523904,"state":"None","activeStates":[],"recoveringStates":[],"pendingStates":[],"happiness":"Happy","isPlayer":true,"lastUpdate":1616625843},{"id":9276,"name":"Independents of Sanos","allegiance":"Federation","government":"Democracy","influence":0.167331,"state":"None","activeStates":[],"recoveringStates":[],"pendingStates":[],"happiness":"Happy","isPlayer":false,"lastUpdate":1616625843}],"stations":[{"id":79936,"marketId":3705068032,"type":"Fleet Carrier","name":"K9T-38W","distanceToArrival":296.807847,"allegiance":"Independent","government":"Fleet Carrier","economy":"Fleet Carrier","secondEconomy":null,"haveMarket":true,"haveShipyard":false,"haveOutfitting":false,"otherServices":["Black Market","Restock","Refuel","Repair","Contacts","Universal Cartographics","Crew Lounge"],"updateTime":{"information":"2021-03-24 21:49:02","market":"2021-03-24 21:49:10","shipyard":null,"outfitting":null}},{"id":23320,"marketId":3228284928,"type":"Orbis Starport","name":"Apgar Terminal","distanceToArrival":296.864456,"allegiance":"Federation","government":"Democracy","economy":"Terraforming","secondEconomy":null,"haveMarket":true,"haveShipyard":true,"haveOutfitting":true,"otherServices":["Black Market","Restock","Refuel","Repair","Contacts","Universal Cartographics","Missions","Crew Lounge","Tuning","Search and Rescue"],"controllingFaction":{"id":9276,"name":"Independents of Sanos"},"updateTime":{"information":"2021-03-24 21:57:14","market":"2021-03-24 21:57:19","shipyard":"2021-03-13 21:01:03","outfitting":"2021-03-24 21:57:19"}}],"bodies":[{"id":9905400,"id64":36032729296409290,"bodyId":1,"name":"Sanos A","type":"Star","subType":"K (Yellow-Orange) Star","parents":[{"Null":0}],"distanceToArrival":0,"isMainStar":true,"isScoopable":true,"age":12692,"spectralClass":"K7","luminosity":"Va","absoluteMagnitude":6.602951,"solarMasses":0.683594,"solarRadius":0.8590392868439971,"surfaceTemperature":4145,"orbitalPeriod":149157.4087628588,"semiMajorAxis":42.891440322239156,"orbitalEccentricity":0.057784,"orbitalInclination":10.679358,"argOfPeriapsis":167.826234,"rotationalPeriod":3.4478455151967595,"rotationalPeriodTidallyLocked":false,"axialTilt":null,"updateTime":"2021-02-23 11:31:41"},{"id":9905405,"id64":216176714391229120,"bodyId":6,"name":"Sanos A 1","type":"Planet","subType":"Earth-like world","parents":[{"Null":5},{"Star":1},{"Null":0}],"distanceToArrival":297,"isLandable":false,"gravity":0.9482140560624709,"earthMasses":0.691103,"radius":5445.0605,"surfaceTemperature":276,"surfacePressure":2.3014628053293857,"volcanismType":"No volcanism","atmosphereType":"Suitable for water-based life","atmosphereComposition":{"Nitrogen":91.25,"Oxygen":8.69,"Carbon dioxide":0.06},"solidComposition":{"Rock":67.21,"Metal":32.79,"Ice":0},"terraformingState":"Terraformed","orbitalPeriod":15.02531201199074,"semiMajorAxis":0.0007772675439048879,"orbitalEccentricity":0.105683,"orbitalInclination":0.412458,"argOfPeriapsis":182.25922,"rotationalPeriod":23.04342612039352,"rotationalPeriodTidallyLocked":false,"axialTilt":-0.163319,"updateTime":"2021-03-14 03:49:33"}],"date":"2017-02-23 01:23:25"}]
        "#};

        assert_eq!(
            parse(example_json.as_bytes())
                .unwrap()
                .get(0)
                .unwrap()
                .name(),
            "Sanos"
        )
    }
}
