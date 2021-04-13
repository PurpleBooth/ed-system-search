use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coords {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub trait System {
    fn name(&self) -> &str;
    fn allegiance(&self) -> String;
    fn stations(&self) -> Vec<Box<dyn Station>>;
    fn factions(&self) -> Vec<Box<dyn Faction>>;
    fn coordinates(&self) -> Coords;
    fn population(&self) -> u128;
}

pub trait Station {
    fn station_type(&self) -> &str;
}

pub trait Faction {
    fn is_player(&self) -> bool;
}

#[derive(Debug, PartialEq, Clone)]
pub enum SystemFilter {
    Allegiance(String),
    MaximumDistanceFrom(Coords, f64),
    MinimumStationCount(HashSet<String>, usize),
    MaximumFactionCount(usize),
    MinimumPopulation(u128),
    ExcludeSystems(HashSet<String>),
    ExcludeSystemsWithPlayerFactions,
}

pub(crate) fn allegiance(allegiance: String) -> SystemFilter {
    SystemFilter::Allegiance(allegiance)
}

pub(crate) fn min_large_docks(count: usize) -> SystemFilter {
    SystemFilter::MinimumStationCount(
        vec![
            String::from("Asteroid base"),
            String::from("Coriolis Starport"),
            String::from("Ocellus Starport"),
            String::from("Orbis Starport"),
        ]
        .into_iter()
        .collect(),
        count,
    )
}

pub(crate) fn min_docks(count: usize) -> SystemFilter {
    SystemFilter::MinimumStationCount(
        vec![
            String::from("Asteroid base"),
            String::from("Coriolis Starport"),
            String::from("Ocellus Starport"),
            String::from("Orbis Starport"),
            String::from("Outpost"),
            String::from("Planetary Outpost"),
            String::from("Planetary Port"),
        ]
        .into_iter()
        .collect(),
        count,
    )
}

pub(crate) fn max_distance_from_sol(light_seconds: f64) -> SystemFilter {
    SystemFilter::MaximumDistanceFrom(
        Coords {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        light_seconds,
    )
}

pub(crate) fn max_distance_from_reference(reference: Coords, light_seconds: f64) -> SystemFilter {
    SystemFilter::MaximumDistanceFrom(reference, light_seconds)
}

pub(crate) fn min_population(count: u128) -> SystemFilter {
    SystemFilter::MinimumPopulation(count)
}

pub(crate) fn min_starports(count: usize) -> SystemFilter {
    SystemFilter::MinimumStationCount(
        vec![
            String::from("Coriolis Starport"),
            String::from("Ocellus Starport"),
            String::from("Orbis Starport"),
        ]
        .into_iter()
        .collect(),
        count,
    )
}

pub(crate) fn exclude_permit_locked() -> SystemFilter {
    SystemFilter::ExcludeSystems(
        vec![
            String::from("Sol"),
            String::from("Beta Hydri"),
            String::from("Vega"),
            String::from("PLX 695"),
            String::from("Ross 128"),
            String::from("Exbeur"),
            String::from("Hors"),
            String::from("HIP 54530"),
            String::from("4 Sextantis"),
            String::from("CD-44 1695"),
            String::from("HIP 22460"),
            String::from("LFT 509"),
            String::from("Mingfu"),
            String::from("Witch's Reach"),
            String::from("Achenar"),
            String::from("Summerland"),
            String::from("Facece"),
            String::from("Alioth"),
            String::from("Shinrarta Dezhra"),
            String::from("CD-43 11917"),
            String::from("Crom"),
            String::from("Jotun"),
            String::from("Terra Mater"),
            String::from("Sirius"),
            String::from("Isinor"),
            String::from("Hodack"),
            String::from("LTT 198"),
            String::from("Luyten 347-14"),
            String::from("Nastrond"),
            String::from("Peregrina"),
            String::from("Pi Mensae"),
            String::from("Tiliala"),
            String::from("van Maanen's Star"),
            String::from("Alpha Hydri"),
            String::from("Bellica"),
            String::from("Dryio Flyuae IC-B c1-377"),
            String::from("HIP 10332"),
            String::from("HIP 104941"),
            String::from("HIP 22182"),
            String::from("HIP 39425"),
            String::from("HIP 51073"),
            String::from("HIP 87621"),
            String::from("HR 4413"),
            String::from("LHS 2894"),
            String::from("LHS 2921"),
            String::from("LHS 3091"),
            String::from("Mbooni"),
            String::from("Plaa Ain HA-Z d46"),
            String::from("Polaris"),
            String::from("Ross 354"),
            String::from("Scheau Bli NB-O d6-1409"),
            String::from("Wolf 262"),
            String::from("Diso 5 C"),
            String::from("Lave 2"),
            String::from("Moon"),
            String::from("Triton"),
            String::from("Azoth"),
            String::from("Dromi"),
            String::from("Lia Fail"),
            String::from("Matet"),
            String::from("Orna"),
            String::from("Otegine"),
            String::from("Sharur"),
            String::from("Tarnkappe"),
            String::from("Tyet"),
            String::from("Wolfsegen"),
        ]
        .into_iter()
        .collect(),
    )
}

#[allow(clippy::too_many_lines)]
pub(crate) fn exclude_rare_commodity() -> SystemFilter {
    SystemFilter::ExcludeSystems(
        vec![
            String::from("47 Ceti"),
            String::from("Aganippe"),
            String::from("Alacarakmo"),
            String::from("Quechua"),
            String::from("Altair"),
            String::from("Alya"),
            String::from("Anduliga"),
            String::from("Any Na"),
            String::from("Arouca"),
            String::from("AZ Cancri"),
            String::from("Leesti"),
            String::from("BaltahSine"),
            String::from("Banki"),
            String::from("Bast"),
            String::from("Belalans"),
            String::from("Borasetani"),
            String::from("HIP 59533"),
            String::from("CD-75 661"),
            String::from("Alpha Centauri"),
            String::from("Heike"),
            String::from("LDS 883"),
            String::from("Aegaeon"),
            String::from("Cherbones"),
            String::from("Chi Eridani"),
            String::from("Coquim"),
            String::from("Crom"),
            String::from("Bento"),
            String::from("Damna"),
            String::from("Delta Phoenicis"),
            String::from("Deuringas"),
            String::from("Diso"),
            String::from("Aerial"),
            String::from("Eleu"),
            String::from("Eranin"),
            String::from("Eshu"),
            String::from("Esuseku"),
            String::from("Ethgreze"),
            String::from("Fujin"),
            String::from("LHS 3447"),
            String::from("Geawen"),
            String::from("Geras"),
            String::from("Irukama"),
            String::from("Phiagre"),
            String::from("Gilya"),
            String::from("Goman"),
            String::from("Haiden"),
            String::from("Havasupai"),
            String::from("Helvetitj"),
            String::from("HIP 10175"),
            String::from("HIP 118311"),
            String::from("HIP 80364"),
            String::from("HIP 41181"),
            String::from("Holva"),
            String::from("LP 375-25"),
            String::from("HR 7221"),
            String::from("Epsilon Indi"),
            String::from("Colonia"),
            String::from("Jaradharre"),
            String::from("Jaroua"),
            String::from("Jotun"),
            String::from("Kachirigin"),
            String::from("Kamitra"),
            String::from("Kamorin"),
            String::from("Karetii"),
            String::from("Karsuki Ti"),
            String::from("Kinago"),
            String::from("Kongga"),
            String::from("Korro Kung"),
            String::from("Lave"),
            String::from("Zaonce"),
            String::from("Hecate"),
            String::from("LTT 9360"),
            String::from("Tanmark"),
            String::from("Noti"),
            String::from("Mechucos"),
            String::from("Medb"),
            String::from("Mokojing"),
            String::from("Momus Reach"),
            String::from("Dea Motrona"),
            String::from("Mukusubii"),
            String::from("Mulachi"),
            String::from("Neritus"),
            String::from("Ngadandari"),
            String::from("Nguna"),
            String::from("Njangari"),
            String::from("LTT 8517"),
            String::from("Ochoeng"),
            String::from("Kappa Fornacis"),
            String::from("Xelabara"),
            String::from("HIP 112974"),
            String::from("36 Ophiuchi"),
            String::from("Orrere"),
            String::from("George Pantazis"),
            String::from("Delta Pavonis"),
            String::from("Njambalba"),
            String::from("Rajukru"),
            String::from("Rapa Bao"),
            String::from("Rusani"),
            String::from("Sanuma"),
            String::from("Arque"),
            String::from("Ngurii"),
            String::from("Sothis"),
            String::from("Tarach Tor"),
            String::from("Terra Mater"),
            String::from("Thrutis"),
            String::from("Tiolce"),
            String::from("Toxandji"),
            String::from("17 Lyrae"),
            String::from("Uszaa"),
            String::from("Utgaroar"),
            String::from("Uzumoku"),
            String::from("V1090 Herculis"),
            String::from("Vanayequi"),
            String::from("Vega"),
            String::from("Vidavanta"),
            String::from("LFT 1421"),
            String::from("Volkhab"),
            String::from("Shinrarta Dezhra"),
            String::from("Wheemete"),
            String::from("Witchhaul"),
            String::from("Wolf 1301"),
            String::from("Wulpa"),
            String::from("Wuthielo Ku"),
            String::from("Xihe"),
            String::from("Yaso Kondi"),
        ]
        .into_iter()
        .collect(),
    )
}

pub(crate) fn max_number_of_factions(count: usize) -> SystemFilter {
    SystemFilter::MaximumFactionCount(count)
}

pub(crate) fn exclude_player_faction() -> SystemFilter {
    SystemFilter::ExcludeSystemsWithPlayerFactions
}
