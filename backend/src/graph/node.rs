use serde::{Deserialize, Serialize};

use crate::osm::Coordinates;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: i64,
    pub coordinates: Coordinates,
    pub meta: Meta,
}

impl Node {
    pub fn new(id: i64, coordinates: Coordinates, meta: Meta) -> Self {
        Self {
            id,
            coordinates,
            meta,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub elevation: Option<f32>,
}

impl Meta {
    pub fn new(elevation: Option<f32>) -> Self {
        Self {
            elevation
        }
    }
}
