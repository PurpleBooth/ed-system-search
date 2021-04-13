use std::io::Read;

use serde::Deserialize;
use thiserror::Error as ThisError;

use crate::domain;
use crate::domain::Coords as DomainCoords;

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Faction {
    #[serde(rename = "isPlayer")]
    pub(crate) is_player: bool,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Station {
    #[serde(rename = "type")]
    pub(crate) station_type: String,
    #[serde(rename = "distanceToArrival")]
    pub(crate) distance_to_arrival: Option<f64>,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ControllingFaction {
    pub(crate) allegiance: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Coords {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct System {
    pub(crate) name: String,
    pub(crate) coords: Coords,
    pub(crate) population: Option<u128>,
    #[serde(rename = "controllingFaction")]
    pub(crate) controlling_faction: ControllingFaction,
    pub(crate) factions: Option<Vec<Faction>>,
    pub(crate) stations: Option<Vec<Station>>,
    pub(crate) allegiance: Option<String>,
    pub(crate) government: Option<String>,
}

impl domain::System for System {
    fn name(&self) -> &str {
        &self.name
    }

    fn allegiance(&self) -> String {
        self.allegiance.clone().unwrap_or_else(|| String::from(""))
    }

    fn government(&self) -> String {
        self.government.clone().unwrap_or_else(|| String::from(""))
    }

    fn stations(&self) -> Vec<Box<dyn domain::Station>> {
        self.stations
            .clone()
            .map(|stations| {
                stations
                    .into_iter()
                    .map(|x| -> Box<dyn domain::Station> { Box::from(x) })
                    .collect::<Vec<Box<dyn domain::Station>>>()
            })
            .unwrap_or_default()
    }

    fn factions(&self) -> Vec<Box<dyn domain::Faction>> {
        self.factions
            .clone()
            .map(|factions| {
                factions
                    .into_iter()
                    .map(|x| -> Box<dyn domain::Faction> { Box::from(x) })
                    .collect::<Vec<Box<dyn domain::Faction>>>()
            })
            .unwrap_or_default()
    }

    fn coordinates(&self) -> DomainCoords {
        DomainCoords {
            x: self.coords.x,
            y: self.coords.y,
            z: self.coords.z,
        }
    }

    fn population(&self) -> u128 {
        self.population.unwrap_or(0)
    }
}

impl domain::Station for Station {
    fn station_type(&self) -> &str {
        self.station_type.as_str()
    }
}

impl domain::Faction for Faction {
    fn is_player(&self) -> bool {
        self.is_player
    }
}

pub(crate) fn parse<R: Read>(file: R) -> Result<Vec<System>, Error> {
    serde_json::from_reader::<_, _>(file).map_err(Error::Parse)
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse edsm data dump: {0:?}")]
    Parse(#[from] serde_json::Error),
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
                allegiance: Some("Federation".parse().unwrap()),
                government: Some("Corporate".parse().unwrap())
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
