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
}

impl domain::System for System {
    fn name(&self) -> &str {
        &self.name
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

    fn coordinates(&self) -> DomainCoords {
        DomainCoords {
            x: self.coords.x,
            y: self.coords.y,
            z: self.coords.z,
        }
    }
}

impl domain::Station for Station {
    fn station_type(&self) -> &str {
        self.station_type.as_str()
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
