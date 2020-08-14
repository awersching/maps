use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::router::grid;
use crate::osm::Coordinates;
use crate::osm::pbf::Pbf;

pub mod node;
pub mod edge;
pub mod router;

pub type Cells = HashMap<Coordinates, Vec<usize>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    offsets: Vec<usize>,
    edges: Vec<Edge>,
    cells: Cells,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, offsets: Vec<usize>, edges: Vec<Edge>) -> Self {
        let cells = grid::create(&nodes);
        Self {
            nodes,
            edges,
            offsets,
            cells,
        }
    }

    pub fn from_pbf(filename: &str) -> Self {
        Pbf::new(filename).read()
    }

    pub fn from_bin(filename: &str) -> Self {
        debug!("Reading graph from {}...", filename);
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let graph: Self = bincode::deserialize_from(reader).unwrap();
        debug!("Read graph from {}...", filename);
        graph
    }

    pub fn save(&self, filename: &str) {
        debug!("Writing graph to {}...", filename);
        let mut bin = File::create(filename).unwrap();
        let encoded = bincode::serialize(self).unwrap();
        bin.write_all(&encoded).unwrap();
        debug!("Wrote graph to {}", filename);
    }

    pub fn node(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    pub fn coordinates(&self, index: usize) -> &Coordinates {
        &self.node(index).coordinates
    }

    pub fn edges(&self, node_index: usize) -> &[Edge] {
        let start = self.offsets[node_index];
        let end = self.offsets[node_index + 1];
        &self.edges[start..end]
    }
}
