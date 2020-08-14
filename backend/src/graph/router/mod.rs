use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;

use crate::graph::{Edge, Graph};
use crate::graph::router::options::Params;
use crate::graph::router::options::Routing::Time;
use crate::graph::router::options::Transport::Car;
use crate::graph::router::route::{Route, RouteBuilder};
use crate::osm::Coordinates;

pub mod grid;
pub mod route;
pub mod options;

pub struct Router<'a> {
    graph: &'a Graph,
    params: Params,

    queue: BinaryHeap<RouterNode>,
    cost: Vec<u32>,
    prev: Vec<Option<&'a Edge>>,
}

impl<'a> Router<'a> {
    pub fn new(graph: &'a Graph, params: Params) -> Self {
        let mut prev = Vec::with_capacity(graph.nodes.len());
        prev.resize(graph.nodes.len(), None);
        Self {
            graph,
            params,

            queue: BinaryHeap::with_capacity(graph.nodes.len()),
            cost: vec![u32::max_value(); graph.nodes.len()],
            prev,
        }
    }

    pub fn shortest_path(&mut self, start: &Coordinates, goal: &Coordinates) -> Result<Route, &str> {
        let start_index = self.graph.nearest_neighbor(start, &self.params)?;
        let start_id = self.graph.node(start_index).id;
        let goal_index = self.graph.nearest_neighbor(goal, &self.params)?;
        let goal_id = self.graph.node(goal_index).id;
        if start_id == goal_id {
            return Err("No path found, start is goal");
        }

        self.cost[start_index] = 0;
        self.queue.push(RouterNode::new(start_index, 0, 0));
        while let Some(node) = self.queue.pop() {
            let id = self.graph.node(node.index).id;
            if id == goal_id {
                let route = RouteBuilder::new(self.graph, &self.prev, self.params.transport)
                    .build(start_index, node.index);
                return Ok(route);
            }
            // better solution already found
            if node.cost > self.cost[node.index] {
                continue;
            }

            for edge in self.graph.edges(node.index) {
                if !edge.is_relevant(&self.params) {
                    continue;
                }

                let cost = node.cost +
                    edge.cost(self.params.transport, self.params.routing);
                if cost < self.cost[edge.target_index] {
                    let heuristic = self.heuristic(edge.target_index, goal_index);
                    let next = RouterNode::new(edge.target_index, cost, heuristic);
                    let _ = mem::replace(&mut self.prev[next.index], Some(edge));
                    self.cost[next.index] = next.cost;
                    self.queue.push(next);
                }
            }
        }
        Err("No path found")
    }

    fn heuristic(&self, from: usize, to: usize) -> u32 {
        if self.params.transport == Car && self.params.routing == Time {
            0
        } else {
            self.graph.coordinates(from)
                .distance(self.graph.coordinates(to)).round() as u32
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RouterNode {
    index: usize,
    cost: u32,
    heuristic: u32,
}

impl RouterNode {
    fn new(index: usize, cost: u32, heuristic: u32) -> Self {
        Self {
            index,
            cost,
            heuristic,
        }
    }

    fn priority(&self) -> u32 {
        self.cost + self.heuristic
    }
}

impl Ord for RouterNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority().cmp(&self.priority())
    }
}

impl PartialOrd for RouterNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use crate::graph::router::RouterNode;

    #[test]
    fn min_priority_queue() {
        let mut queue = BinaryHeap::with_capacity(5);
        queue.push(RouterNode::new(3, 3, 0));
        queue.push(RouterNode::new(1, 1, 0));
        queue.push(RouterNode::new(20, 20, 0));
        queue.push(RouterNode::new(2, 2, 0));
        queue.push(RouterNode::new(5, 5, 0));

        assert_eq!(queue.pop().unwrap().cost, 1);
        assert_eq!(queue.pop().unwrap().cost, 2);
        assert_eq!(queue.pop().unwrap().cost, 3);
        assert_eq!(queue.pop().unwrap().cost, 5);
        queue.push(RouterNode::new(15, 15, 0));
        assert_eq!(queue.pop().unwrap().cost, 15);
        assert_eq!(queue.pop().unwrap().cost, 20);
    }
}
