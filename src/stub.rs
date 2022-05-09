use crate::domain;

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub(crate) stations: Vec<Station>,
    pub(crate) factions: Vec<Faction>,
    pub(crate) coords: crate::domain::Coords,
    pub(crate) name: String,
    pub(crate) population: u128,
    pub allegiance: String,
    pub government: String,
}

impl<'system> domain::System<'system> for System {
    fn name(&self) -> &str {
        &self.name
    }

    fn allegiance(&self) -> &str {
        &self.allegiance
    }

    fn government(&self) -> &str {
        &self.government
    }

    fn stations(&self) -> Vec<Box<dyn domain::Station>> {
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

    fn coordinates(&self) -> domain::Coords {
        self.coords
    }

    fn population(&self) -> u128 {
        self.population
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Station {
    pub(crate) station_type: Option<String>,
}

impl domain::Station for Station {
    fn station_type(&self) -> Option<&str> {
        self.station_type.as_deref()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Faction {
    pub(crate) is_player: bool,
}

impl crate::domain::Faction for Faction {
    fn is_player(&self) -> bool {
        self.is_player
    }
}
