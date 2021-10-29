use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coords {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub trait System<'system> {
    fn name(&self) -> &str;
    fn allegiance(&self) -> &str;
    fn government(&self) -> &str;
    fn stations(&self) -> Vec<Box<dyn Station>>;
    fn factions(&self) -> Vec<Box<dyn Faction>>;
    fn coordinates(&self) -> Coords;
    fn population(&self) -> u128;
}

pub trait Station {
    fn station_type(&self) -> Option<&str>;
}

pub trait Faction {
    fn is_player(&self) -> bool;
}

#[derive(Debug, PartialEq, Clone)]
pub enum SystemFilter<'a> {
    Allegiance(&'a str),
    Government(&'a str),
    MaximumDistanceFrom(Coords, f64),
    MinimumStationCount(HashSet<&'a str>, usize),
    MaximumFactionCount(usize),
    MinimumPopulation(u128),
    ExcludeSystems(HashSet<&'a str>),
    ExcludeSystemsWithPlayerFactions,
}

pub const fn allegiance(allegiance: &str) -> SystemFilter {
    SystemFilter::Allegiance(allegiance)
}

pub const fn government(government: &str) -> SystemFilter {
    SystemFilter::Government(government)
}

pub fn min_large_docks<'a>(count: usize) -> SystemFilter<'a> {
    SystemFilter::MinimumStationCount(
        vec![
            "Asteroid base",
            "Coriolis Starport",
            "Ocellus Starport",
            "Orbis Starport",
        ]
        .into_iter()
        .collect(),
        count,
    )
}

pub fn min_docks<'a>(count: usize) -> SystemFilter<'a> {
    SystemFilter::MinimumStationCount(
        vec![
            ("Asteroid base"),
            ("Coriolis Starport"),
            ("Ocellus Starport"),
            ("Orbis Starport"),
            ("Outpost"),
            ("Planetary Outpost"),
            ("Planetary Port"),
        ]
        .into_iter()
        .collect(),
        count,
    )
}

pub const fn max_distance_from_sol<'a>(light_seconds: f64) -> SystemFilter<'a> {
    SystemFilter::MaximumDistanceFrom(
        Coords {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        light_seconds,
    )
}

pub const fn max_distance_from_reference<'a>(
    reference: Coords,
    light_seconds: f64,
) -> SystemFilter<'a> {
    SystemFilter::MaximumDistanceFrom(reference, light_seconds)
}

pub const fn min_population<'a>(count: u128) -> SystemFilter<'a> {
    SystemFilter::MinimumPopulation(count)
}

pub fn min_starports<'a>(count: usize) -> SystemFilter<'a> {
    SystemFilter::MinimumStationCount(
        vec![
            ("Coriolis Starport"),
            ("Ocellus Starport"),
            ("Orbis Starport"),
        ]
        .into_iter()
        .collect(),
        count,
    )
}

pub fn exclude_permit_locked<'a>() -> SystemFilter<'a> {
    SystemFilter::ExcludeSystems(
        vec![
            ("Sol"),
            ("Beta Hydri"),
            ("Vega"),
            ("PLX 695"),
            ("Ross 128"),
            ("Exbeur"),
            ("Hors"),
            ("HIP 54530"),
            ("4 Sextantis"),
            ("CD-44 1695"),
            ("HIP 22460"),
            ("LFT 509"),
            ("Mingfu"),
            ("Witch's Reach"),
            ("Achenar"),
            ("Summerland"),
            ("Facece"),
            ("Alioth"),
            ("Shinrarta Dezhra"),
            ("CD-43 11917"),
            ("Crom"),
            ("Jotun"),
            ("Terra Mater"),
            ("Sirius"),
            ("Isinor"),
            ("Hodack"),
            ("LTT 198"),
            ("Luyten 347-14"),
            ("Nastrond"),
            ("Peregrina"),
            ("Pi Mensae"),
            ("Tiliala"),
            ("van Maanen's Star"),
            ("Alpha Hydri"),
            ("Bellica"),
            ("Dryio Flyuae IC-B c1-377"),
            ("HIP 10332"),
            ("HIP 104941"),
            ("HIP 22182"),
            ("HIP 39425"),
            ("HIP 51073"),
            ("HIP 87621"),
            ("HR 4413"),
            ("LHS 2894"),
            ("LHS 2921"),
            ("LHS 3091"),
            ("Mbooni"),
            ("Plaa Ain HA-Z d46"),
            ("Polaris"),
            ("Ross 354"),
            ("Scheau Bli NB-O d6-1409"),
            ("Wolf 262"),
            ("Diso 5 C"),
            ("Lave 2"),
            ("Moon"),
            ("Triton"),
            ("Azoth"),
            ("Dromi"),
            ("Lia Fail"),
            ("Matet"),
            ("Orna"),
            ("Otegine"),
            ("Sharur"),
            ("Tarnkappe"),
            ("Tyet"),
            ("Wolfsegen"),
        ]
        .into_iter()
        .collect(),
    )
}

#[allow(clippy::too_many_lines)]
pub fn exclude_rare_commodity<'a>() -> SystemFilter<'a> {
    SystemFilter::ExcludeSystems(
        vec![
            ("47 Ceti"),
            ("Aganippe"),
            ("Alacarakmo"),
            ("Quechua"),
            ("Altair"),
            ("Alya"),
            ("Anduliga"),
            ("Any Na"),
            ("Arouca"),
            ("AZ Cancri"),
            ("Leesti"),
            ("BaltahSine"),
            ("Banki"),
            ("Bast"),
            ("Belalans"),
            ("Borasetani"),
            ("HIP 59533"),
            ("CD-75 661"),
            ("Alpha Centauri"),
            ("Heike"),
            ("LDS 883"),
            ("Aegaeon"),
            ("Cherbones"),
            ("Chi Eridani"),
            ("Coquim"),
            ("Crom"),
            ("Bento"),
            ("Damna"),
            ("Delta Phoenicis"),
            ("Deuringas"),
            ("Diso"),
            ("Aerial"),
            ("Eleu"),
            ("Eranin"),
            ("Eshu"),
            ("Esuseku"),
            ("Ethgreze"),
            ("Fujin"),
            ("LHS 3447"),
            ("Geawen"),
            ("Geras"),
            ("Irukama"),
            ("Phiagre"),
            ("Gilya"),
            ("Goman"),
            ("Haiden"),
            ("Havasupai"),
            ("Helvetitj"),
            ("HIP 10175"),
            ("HIP 118311"),
            ("HIP 80364"),
            ("HIP 41181"),
            ("Holva"),
            ("LP 375-25"),
            ("HR 7221"),
            ("Epsilon Indi"),
            ("Colonia"),
            ("Jaradharre"),
            ("Jaroua"),
            ("Jotun"),
            ("Kachirigin"),
            ("Kamitra"),
            ("Kamorin"),
            ("Karetii"),
            ("Karsuki Ti"),
            ("Kinago"),
            ("Kongga"),
            ("Korro Kung"),
            ("Lave"),
            ("Zaonce"),
            ("Hecate"),
            ("LTT 9360"),
            ("Tanmark"),
            ("Noti"),
            ("Mechucos"),
            ("Medb"),
            ("Mokojing"),
            ("Momus Reach"),
            ("Dea Motrona"),
            ("Mukusubii"),
            ("Mulachi"),
            ("Neritus"),
            ("Ngadandari"),
            ("Nguna"),
            ("Njangari"),
            ("LTT 8517"),
            ("Ochoeng"),
            ("Kappa Fornacis"),
            ("Xelabara"),
            ("HIP 112974"),
            ("36 Ophiuchi"),
            ("Orrere"),
            ("George Pantazis"),
            ("Delta Pavonis"),
            ("Njambalba"),
            ("Rajukru"),
            ("Rapa Bao"),
            ("Rusani"),
            ("Sanuma"),
            ("Arque"),
            ("Ngurii"),
            ("Sothis"),
            ("Tarach Tor"),
            ("Terra Mater"),
            ("Thrutis"),
            ("Tiolce"),
            ("Toxandji"),
            ("17 Lyrae"),
            ("Uszaa"),
            ("Utgaroar"),
            ("Uzumoku"),
            ("V1090 Herculis"),
            ("Vanayequi"),
            ("Vega"),
            ("Vidavanta"),
            ("LFT 1421"),
            ("Volkhab"),
            ("Shinrarta Dezhra"),
            ("Wheemete"),
            ("Witchhaul"),
            ("Wolf 1301"),
            ("Wulpa"),
            ("Wuthielo Ku"),
            ("Xihe"),
            ("Yaso Kondi"),
        ]
        .into_iter()
        .collect(),
    )
}

pub const fn max_number_of_factions<'a>(count: usize) -> SystemFilter<'a> {
    SystemFilter::MaximumFactionCount(count)
}

pub const fn exclude_player_faction<'a>() -> SystemFilter<'a> {
    SystemFilter::ExcludeSystemsWithPlayerFactions
}
