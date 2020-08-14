use crate::osm::Coordinates;
use crate::srtm::cache::Cache;

mod cache;

const VOID: i16 = -32768;

pub struct Srtm {
    cache: Cache
}

impl Srtm {
    pub fn new() -> Self {
        Self {
            cache: Cache::new()
        }
    }

    pub fn elevation(&mut self, coords: &Coordinates) -> Option<f32> {
        let tile = self.cache.get(coords);
        let row = ((tile.lat() + 1.0 - coords.lat()) * (tile.square_side - 1) as f64).floor();
        let column = ((coords.lon() - tile.lon()) * (tile.square_side - 1) as f64).floor();
        let center = tile.coordinates(row, column);
        if coords.lat == center.lat && coords.lon == center.lon {
            return tile.elevation(row, column).map(|e| e as f32);
        }

        // interpolate using sum((1 / distance) * elevation) / sum(1 / distance)
        let offsets = [
            (0.0, 0.0),
            (1.0, 0.0),
            (-1.0, 0.0),
            (0.0, 1.0),
            (0.0, -1.0),
            (1.0, 1.0),
            (-1.0, -1.0),
            (1.0, -1.0),
            (-1.0, 1.0),
        ];
        let mut weights = 0.0;
        let mut elevation = 0.0;
        for (r, c) in &offsets {
            let e = tile.elevation(row + *r, column + *c);

            if let Some(e) = e {
                let neighbor = Coordinates::from(
                    center.lat() + r / (tile.square_side - 1) as f64,
                    center.lon() + c / (tile.square_side - 1) as f64,
                );
                let distance = coords.distance(&neighbor) as f32;
                weights += 1.0 / distance;
                elevation += e as f32 / distance;
            }
        }
        Some(elevation / weights)
    }
}

pub struct Tile {
    lat: i8,
    lon: i8,
    data: Vec<u8>,
    square_side: i32,
    resolution: f32,
}

impl Tile {
    pub fn new(coords: &Coordinates, data: Vec<u8>) -> Self {
        let square_side = (data.len() as f64 / 2.0).sqrt();
        Self {
            lat: coords.lat() as i8,
            lon: coords.lon() as i8,
            data,
            square_side: square_side as i32,
            resolution: (1.0 / (square_side - 1.0)) as f32,
        }
    }

    fn lat(&self) -> f64 {
        self.lat as f64
    }

    fn lon(&self) -> f64 {
        self.lon as f64
    }

    fn elevation(&self, row: f64, column: f64) -> Option<i16> {
        let index = (row * self.square_side as f64 + column) as usize;
        let start = index * 2;
        let end = start + 1;
        if self.data.get(start).is_none() || self.data.get(end).is_none() {
            return None;
        }

        let bytes = [self.data[start], self.data[end]];
        let elevation = i16::from_be_bytes(bytes);
        if elevation == VOID { None } else { Some(elevation) }
    }

    fn coordinates(&self, row: f64, column: f64) -> Coordinates {
        let lat = self.lat() + 1.0 - row * self.resolution as f64;
        let lon = self.lon() + column * self.resolution as f64;
        Coordinates::from(lat, lon)
    }
}
