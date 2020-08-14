use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};
use std::path::Path;

use log::debug;
use reqwest::blocking::Response;
use zip::ZipArchive;

use crate::osm::Coordinates;
use crate::srtm::Tile;

pub struct Cache {
    index: HashMap<String, String>,
    tiles: HashMap<String, Tile>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            index: index(),
            tiles: HashMap::new(),
        }
    }

    pub fn get(&mut self, coords: &Coordinates) -> &Tile {
        let filename = filename(coords);

        match self.tiles.entry(filename.clone()) {
            Entry::Occupied(entry) => {
                entry.into_mut()
            }
            Entry::Vacant(entry) => {
                let bytes = if Path::new(&filename).exists() {
                    open(&filename)
                } else {
                    let url = self.index.get(entry.key()).unwrap();
                    download(&filename, url)
                };
                let tile = Tile::new(coords, bytes);
                entry.insert(tile)
            }
        }
    }
}

fn index() -> HashMap<String, String> {
    let json_str = include_str!("srtm-index.json");
    serde_json::from_str(json_str).unwrap()
}

fn filename(coords: &Coordinates) -> String {
    let lat = coords.lat() as i32;
    let lon = coords.lon() as i32;

    let lat_padded = format!("{:0width$}", lat, width = 2);
    let lon_padded = format!("{:0width$}", lon, width = 3);
    let lat_cardinal = if lat >= 0 { "N" } else { "S" };
    let lon_cardinal = if lon >= 0 { "E" } else { "W" };
    format!("{}{}{}{}.hgt", lat_cardinal, lat_padded, lon_cardinal, lon_padded)
}

fn open(filename: &str) -> Vec<u8> {
    debug!("SRTM file {} exists, reading from disk...", filename);
    fs::read(filename).unwrap()
}

fn download(filename: &str, url: &str) -> Vec<u8> {
    debug!("Downloading SRTM file {}...", filename);
    let content = reqwest::blocking::get(url).unwrap();
    let zip = zip(content);
    let bytes = unzip(zip, filename);
    fs::write(&filename, &bytes).unwrap();
    bytes
}

fn zip(mut content: Response) -> ZipArchive<Cursor<Vec<u8>>> {
    let mut buf = Vec::new();
    content.read_to_end(&mut buf).unwrap();
    let reader = Cursor::new(buf);
    ZipArchive::new(reader).unwrap()
}

fn unzip(mut zip: ZipArchive<Cursor<Vec<u8>>>, filename: &str) -> Vec<u8> {
    let mut hgt = zip.by_name(filename).unwrap();
    let mut buf = Vec::new();
    hgt.read_to_end(&mut buf).unwrap();
    buf
}
