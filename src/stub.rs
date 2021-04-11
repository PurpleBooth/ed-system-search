use crate::domain;

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub(crate) stations: Vec<Station>,
    pub(crate) factions: Vec<Faction>,
    pub(crate) coords: crate::domain::Coords,
    pub(crate) name: String,
    pub(crate) population: u128,
}

impl crate::domain::System for System {
    fn name(&self) -> &str {
        &self.name
    }

    fn stations(&self) -> Vec<Box<dyn crate::domain::Station>> {
        self.stations
            .clone()
            .into_iter()
            .map(|x| -> Box<dyn domain::Station> { Box::from(x) })
            .collect::<Vec<Box<dyn domain::Station>>>()
    }

    fn factions(&self) -> Vec<Box<dyn domain::Faction>> {
        self.factions
            .clone()
            .into_iter()
            .map(|x| -> Box<dyn domain::Faction> { Box::from(x) })
            .collect::<Vec<Box<dyn domain::Faction>>>()
    }

    fn coordinates(&self) -> crate::domain::Coords {
        self.coords
    }

    fn population(&self) -> u128 {
        self.population
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Station {
    station_type: String,
}

impl crate::domain::Station for Station {
    fn station_type(&self) -> &str {
        self.station_type.as_str()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Faction {
    is_player: bool,
}

impl crate::domain::Faction for Faction {
    fn is_player(&self) -> bool {
        self.is_player
    }
}
