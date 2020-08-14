use std::cmp::Ordering;

use osmpbfreader::Way;
use serde::{Deserialize, Serialize};

use crate::graph::router::options::{Params, Routing, Transport};
use crate::graph::router::options::Routing::Time;
use crate::graph::router::options::Transport::{Bike, Car, Walk};
use crate::osm::{Highway, Kmh, Surface};
use crate::osm::Highway::{Motorway, MotorwayLink, Primary, PrimaryLink, Secondary,
                          SecondaryLink, Tertiary, TertiaryLink, Trunk, TrunkLink};
use crate::osm::Surface::{Asphalt, Concrete, Paved};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub source_index: usize,
    pub target_index: usize,
    pub distance: Option<u32>,
    pub meta: Meta,
}

impl Edge {
    pub fn new(source_index: usize, target_index: usize, meta: Meta) -> Self {
        Self {
            source_index,
            target_index,
            distance: None,
            meta,
        }
    }

    pub fn cost(&self, mode: Transport, routing: Routing) -> u32 {
        if mode == Car && routing == Time {
            self.meta.max_speed.time(self.distance())
        } else {
            // Bike and Walk are assumed to have constant speed
            self.distance()
        }
    }

    pub fn time(&self, mode: Transport) -> u32 {
        match mode {
            Car => self.meta.max_speed.time(self.distance()),
            Bike => Kmh::new(20).time(self.distance()),
            Walk => Kmh::new(5).time(self.distance()),
            _ => panic!("Unsupported transport mode")
        }
    }

    pub fn distance(&self) -> u32 {
        // distance is always set after parsing is finished
        self.distance.unwrap()
    }

    pub fn transport(&self) -> Transport {
        Transport::from(self.meta.highway)
    }

    pub fn is_paved(&self) -> bool {
        match self.meta.highway {
            Motorway | Trunk | Primary | Secondary | Tertiary |
            MotorwayLink | TrunkLink | PrimaryLink | SecondaryLink | TertiaryLink => return true,
            _ => ()
        }
        if let Some(surface) = self.meta.surface {
            match surface {
                Asphalt | Concrete | Paved => return true,
                _ => ()
            }
        }
        false
    }

    pub fn is_relevant(&self, params: &Params) -> bool {
        let matches_transport = self.transport().contains(params.transport);
        if params.avoid_unpaved {
            matches_transport && self.is_paved()
        } else {
            matches_transport
        }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_index.cmp(&other.source_index)
            .then_with(|| self.target_index.cmp(&other.target_index))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub grade: Option<u8>,
    pub max_speed: Kmh,
    pub highway: Highway,
    pub surface: Option<Surface>,
}

impl Meta {
    pub fn new(way: &Way) -> Result<Self, &str> {
        let highway = Highway::from(&way)
            .ok_or("Way is not a highway")?;
        let max_speed = Kmh::from(&way)
            .or_else(|| highway.default_speed()).unwrap();

        Ok(Self {
            grade: None,
            max_speed,
            highway,
            surface: Surface::from(way),
        })
    }
}
