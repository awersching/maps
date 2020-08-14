use std::collections::HashMap;
use std::fs::File;
use std::mem;
use std::path::Path;

use log::debug;
use osmpbfreader::{NodeId, OsmObj, OsmPbfReader};

use crate::graph::{edge, Graph, node};
use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::osm::{Coordinates, is_oneway};
use crate::srtm::Srtm;

pub struct Pbf<'a> {
    filename: &'a str,
    node_indices: HashMap<NodeId, usize>,
    number_nodes: usize,
}

impl<'a> Pbf<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self {
            filename,
            node_indices: HashMap::new(),
            number_nodes: 0,
        }
    }

    pub fn read(&mut self) -> Graph {
        debug!("Parsing edges...");
        let edges = self.parse_ways();
        debug!("Parsed {} edges", edges.len());
        debug!("Parsing nodes...");
        let nodes = self.parse_nodes();
        debug!("Parsed {} nodes", nodes.capacity());
        debug!("Creating graph...");
        self.create_graph(nodes, edges)
    }

    fn parse_ways(&mut self) -> Vec<Edge> {
        let mut pbf = read_pbf(self.filename);
        let mut edges = Vec::new();

        for object in pbf.par_iter() {
            if let OsmObj::Way(way) = object.unwrap() {
                let meta = if let Ok(meta) = edge::Meta::new(&way) {
                    meta
                } else {
                    continue;
                };
                let is_oneway = is_oneway(&way);

                self.insert_node_id(*way.nodes.get(0).unwrap());
                for i in 1..way.nodes.len() {
                    let source_id = *way.nodes.get(i - 1).unwrap();
                    let source_index = *self.node_indices.get(&source_id).unwrap();
                    let target_id = *way.nodes.get(i).unwrap();
                    self.insert_node_id(target_id);
                    let target_index = *self.node_indices.get(&target_id).unwrap();

                    let edge = Edge::new(source_index, target_index, meta.clone());
                    if !is_oneway {
                        let mut reverse = edge.clone();
                        reverse.source_index = target_index;
                        reverse.target_index = source_index;
                        edges.push(reverse);
                    }
                    edges.push(edge);
                }
            }
        }
        edges.sort();
        edges
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut pbf = read_pbf(self.filename);
        let mut nodes = Vec::with_capacity(self.node_indices.len());
        nodes.resize(self.node_indices.len(), None);
        let mut srtm = Srtm::new();

        for object in pbf.par_iter() {
            if let OsmObj::Node(osm_node) = object.unwrap() {
                let id = osm_node.id;
                if let Some(index) = self.node_indices.remove(&id) {
                    let coordinates = Coordinates::new(
                        osm_node.decimicro_lat,
                        osm_node.decimicro_lon,
                    );
                    let elevation = srtm.elevation(&coordinates);
                    let meta = node::Meta::new(elevation);

                    let node = Node::new(id.0, coordinates, meta);
                    let _ = mem::replace(&mut nodes[index], Some(node));
                }
            }
        }
        nodes.into_iter()
            .map(|n| n.unwrap())
            .collect()
    }

    fn create_graph(&self, nodes: Vec<Node>, mut edges: Vec<Edge>) -> Graph {
        let offsets_len = self.number_nodes + 1;
        let mut offsets = vec![0; offsets_len];

        for edge in &mut edges {
            let source = &nodes[edge.source_index];
            let target = &nodes[edge.target_index];
            edge.distance = Some(source.coordinates
                .distance(&target.coordinates).round() as u32);
            edge.meta.grade = grade(source, target, edge.distance());

            offsets[edge.source_index + 1] += 1;
        }

        for i in 1..offsets.len() {
            offsets[i] += offsets[i - 1]
        }
        Graph::new(nodes, offsets, edges)
    }

    fn insert_node_id(&mut self, id: NodeId) {
        if self.node_indices.contains_key(&id) {
            return;
        }
        self.node_indices.insert(id, self.number_nodes);
        self.number_nodes += 1;
    }
}

fn read_pbf(filename: &str) -> OsmPbfReader<File> {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    OsmPbfReader::new(file)
}

fn grade(source: &Node, target: &Node, distance: u32) -> Option<u8> {
    if let Some(source_e) = source.meta.elevation {
        if let Some(target_e) = target.meta.elevation {
            let rise = (source_e as f32 - target_e as f32).abs();
            let run = distance as f32;
            let grade = (rise / run) * 100.0;

            return Some(grade.round() as u8);
        }
    }
    None
}
