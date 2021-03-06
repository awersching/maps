use std::collections::HashMap;

use crate::graph::{Cells, Graph, Node};
use crate::graph::router::options::Params;
use crate::osm::Coordinates;

pub fn create(nodes: &[Node]) -> Cells {
    let mut cells: Cells = HashMap::with_capacity(nodes.len());

    for (i, node) in nodes.iter().enumerate() {
        let coordinates = node.coordinates.clone();
        if let Some(indices) = cells.get_mut(&coordinates) {
            indices.push(i);
        } else {
            let mut indices = Vec::new();
            indices.push(i);
            cells.insert(coordinates, indices);
        }
    }
    cells
}

impl Graph {
    pub fn nearest_neighbor(&self, coords: &Coordinates, params: &Params) -> Result<usize, &str> {
        let exact_cell = self.cells.get(coords)
            .ok_or("Couldn't locate point on map")?;
        let mut best = self.closest(vec![exact_cell; 1], coords, params);

        // check 10% of the cells at max
        let max_radius = self.cells.len() as f32 * 0.1;
        for radius in 1..max_radius as i32 {
            let adjacent_cells = self.adjacent_cells(coords, radius);
            let adjacent = self.closest(adjacent_cells, coords, params);

            if best.index.is_none() || best.dist > adjacent.dist {
                best = adjacent;
            } else {
                break;
            }
        }
        best.index.ok_or("No point matching transportation found")
    }

    fn adjacent_cells(&self, coords: &Coordinates, radius: i32) -> Vec<&Vec<usize>> {
        let mut cells = Vec::with_capacity((radius * 8) as usize);

        for i in -radius..=radius {
            for j in -radius..=radius {
                if i.abs() != radius && j.abs() != radius {
                    // cells from previous radii (inner cells) are not considered
                    continue;
                }

                let key = Coordinates::from(
                    coords.lat() + i as f64,
                    coords.lon() + j as f64,
                );
                let cell = self.cells.get(&key);
                if cell.is_none() {
                    // cell is outside of pbf file
                    continue;
                }
                cells.push(cell.unwrap());
            }
        }
        cells
    }

    fn closest(&self, cells: Vec<&Vec<usize>>, coords: &Coordinates, params: &Params) -> Neighbor {
        let mut closest = Neighbor::new();

        for cell in cells {
            for i in cell {
                let is_relevant = self.edges(*i).iter()
                    .any(|e| e.is_relevant(params));
                if !is_relevant {
                    continue;
                }

                let dist = self.coordinates(*i).distance(coords).round() as u32;
                if dist < closest.dist {
                    closest.dist = dist;
                    closest.index = Some(*i);
                }
            }
        }
        closest
    }
}

struct Neighbor {
    index: Option<usize>,
    dist: u32,
}

impl Neighbor {
    fn new() -> Self {
        Self {
            index: None,
            dist: u32::max_value(),
        }
    }
}
