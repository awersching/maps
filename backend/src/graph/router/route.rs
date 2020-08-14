use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph::router::options::Transport;
use crate::osm::Coordinates;

pub struct RouteBuilder<'a, 'b> {
    graph: &'a Graph,
    prev: &'b [Option<&'a Edge>],
    transport: Transport,
}

impl<'a, 'b> RouteBuilder<'a, 'b> {
    pub fn new(graph: &'a Graph, prev: &'b [Option<&'a Edge>], transport: Transport) -> Self {
        Self {
            graph,
            prev,
            transport,
        }
    }

    pub fn build(&self, start_index: usize, goal_index: usize) -> Route {
        let mut route = Route::new();

        let mut edge = self.prev[goal_index].unwrap();
        loop {
            route.nodes.push(self.graph.node(edge.target_index).clone());
            route.edges.push(edge.clone());
            route.distance += edge.distance();
            route.time += edge.time(self.transport);
            if self.graph.edges(edge.target_index).len() > 2 &&
                edge.target_index != goal_index {
                // - in  and outgoing edge
                route.intersections += self.graph.edges(edge.target_index).len() - 2;
            }

            edge = self.prev[edge.source_index].unwrap();
            if edge.source_index == start_index {
                route.nodes.push(self.graph.node(edge.source_index).clone());
                break;
            }
        }

        route.nodes.reverse();
        route.edges.reverse();
        route.calc_curvature();
        route
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub time: u32,
    pub distance: u32,
    pub intersections: usize,
    pub curvature: Curvature,
}

impl Route {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            time: 0,
            distance: 0,
            intersections: 0,
            curvature: Curvature::new(),
        }
    }

    pub fn merge(&mut self, mut other: Route) {
        // already included as the goal of previous route
        other.nodes.remove(0);
        self.nodes.extend(other.nodes);
        self.edges.extend(other.edges);
        self.time += other.time;
        self.distance += other.distance;
        self.intersections += other.intersections;
        self.curvature.radii.extend(other.curvature.radii);
        self.curvature.score += other.curvature.score;
    }

    fn calc_curvature(&mut self) {
        self.curvature.radii.push(Radius::gamma(
            &self.nodes[0].coordinates,
            &self.nodes[1].coordinates,
            &self.nodes[2].coordinates,
        ));
        for i in 1..self.nodes.len() - 1 {
            let c1 = &self.nodes[i - 1].coordinates;
            let c2 = &self.nodes[i].coordinates;
            let c3 = &self.nodes[i + 1].coordinates;
            self.curvature.radii.push(Radius::gamma(c1, c2, c3));
        }
        self.curvature.radii.push(Radius::gamma(
            &self.nodes[self.nodes.len() - 3].coordinates,
            &self.nodes[self.nodes.len() - 2].coordinates,
            &self.nodes[self.nodes.len() - 1].coordinates,
        ));

        self.curvature.score = self.curvature.radii.iter()
            .map(|r| r.score())
            .sum();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Curvature {
    pub radii: Vec<Radius>,
    pub score: f32,
}

impl Curvature {
    fn new() -> Self {
        Self {
            radii: Vec::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Radius(Option<f32>);

impl Radius {
    fn gamma(c1: &Coordinates, c2: &Coordinates, c3: &Coordinates) -> Self {
        let (a, b, c) = Self::sides(c1, c2, c3);
        let radians = ((a.powi(2) + b.powi(2) - c.powi(2)) / (2.0 * a * b))
            .acos();
        Self::new(radians)
    }

    fn new(radians: f32) -> Self {
        if radians.is_normal() {
            let degrees = radians * (180.0 / std::f32::consts::PI);
            Self(Some(degrees))
        } else {
            Self(None)
        }
    }

    fn sides(c1: &Coordinates, c2: &Coordinates, c3: &Coordinates) -> (f32, f32, f32) {
        let a = c2.distance(&c3) as f32;
        let b = c1.distance(&c2) as f32;
        let c = c1.distance(&c3) as f32;
        (a, b, c)
    }

    fn score(self) -> f32 {
        if self.0.is_none() {
            return 0.0;
        }
        let radius = self.0.unwrap();

        if radius < 160.0 {
            6.0
        } else if radius < 170.0 {
            2.0
        } else if radius < 175.0 {
            1.0
        } else {
            0.0
        }
    }
}
