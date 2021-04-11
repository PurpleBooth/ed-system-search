#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coords {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub trait System {
    fn name(&self) -> &str;
    fn stations(&self) -> Vec<Box<dyn Station>>;
    fn coordinates(&self) -> Coords;
    fn population(&self) -> u128;
}

pub trait Station {
    fn station_type(&self) -> &str;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SearchOptions {
    pub(crate) min_large_docks: Option<usize>,
    pub(crate) min_docks: Option<usize>,
    pub(crate) max_distance_from_sol: Option<f64>,
    pub(crate) reference: Option<Coords>,
    pub(crate) max_distance_from_reference: Option<f64>,
    pub(crate) min_population: Option<u128>,
    pub(crate) min_starports: Option<usize>,
    pub(crate) exclude_permit_locked: bool,
}
