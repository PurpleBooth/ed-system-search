use crate::domain;

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub(crate) stations: Vec<Station>,
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
