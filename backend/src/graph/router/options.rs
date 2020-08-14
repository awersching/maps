use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::graph::router::options::Transport::{All, Bike, BikeWalk, Car, CarBike, Walk};
use crate::osm::Highway::{Cycleway, Footway, LivingStreet, Motorway, MotorwayLink, Path,
                          Pedestrian, Primary, PrimaryLink, Residential, Road, Secondary,
                          SecondaryLink, Service, Steps,
                          Tertiary, TertiaryLink, Track, Trunk, TrunkLink, Unclassified};
use crate::osm::Highway;

#[derive(Debug, Clone)]
pub struct Params {
    pub transport: Transport,
    pub routing: Routing,
    pub avoid_unpaved: bool,
}

impl Params {
    pub fn new(transport: Transport, routing: Routing, avoid_unpaved: bool) -> Self {
        Self {
            transport,
            routing,
            avoid_unpaved,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Transport {
    Car,
    Bike,
    Walk,

    All,
    CarBike,
    BikeWalk,
}

impl Transport {
    pub fn from(highway: Highway) -> Self {
        match highway {
            Residential | Tertiary | Unclassified | Service | LivingStreet | TertiaryLink => All,
            Secondary | SecondaryLink | Primary | PrimaryLink => CarBike,
            Track | Road => BikeWalk,
            Motorway | MotorwayLink | Trunk | TrunkLink => Car,
            Cycleway => Bike,
            Pedestrian | Footway | Path | Steps => Walk,
        }
    }

    pub fn contains(self, other: Self) -> bool {
        self == All || self == other ||
            (self == CarBike && (other == Car || other == Bike)) ||
            (self == BikeWalk && (other == Bike || other == Walk))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Routing {
    Time,
    Distance,
}