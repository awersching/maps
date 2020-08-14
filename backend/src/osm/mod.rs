use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use osmpbfreader::Way;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serializer, SerializeStruct};
use strum_macros::EnumString;

pub mod pbf;

pub fn is_oneway(way: &Way) -> bool {
    let tag = way.tags.get("oneway");
    // not oneway assumed if not specified
    if tag.is_none() {
        return false;
    }
    tag.unwrap() == "yes"
}

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub lat: i32,
    pub lon: i32,
}

impl Coordinates {
    pub fn new(lat: i32, lon: i32) -> Self {
        Self {
            lat,
            lon,
        }
    }

    pub fn from(lat: f64, lon: f64) -> Self {
        Self {
            lat: (lat / 1e-7) as i32,
            lon: (lon / 1e-7) as i32,
        }
    }

    pub fn lat(&self) -> f64 {
        f64::from(self.lat) * 1e-7
    }

    pub fn lon(&self) -> f64 {
        f64::from(self.lon) * 1e-7
    }

    fn lat_rounded(&self) -> i32 {
        (self.lat() * 10.0).round() as i32
    }

    fn lon_rounded(&self) -> i32 {
        (self.lon() * 10.0).round() as i32
    }

    /// Haversine distance
    pub fn distance(&self, other: &Self) -> f64 {
        let theta1 = self.lon().to_radians();
        let theta2 = other.lon().to_radians();
        let delta_theta = (other.lon() - self.lon()).to_radians();
        let delta_lambda = (other.lat() - self.lat()).to_radians();
        let a = (delta_theta / 2.0).sin().powi(2)
            + theta1.cos() * theta2.cos() * (delta_lambda / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        6_371_000.0 * c
    }
}

impl Eq for Coordinates {}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.lat_rounded() == other.lat_rounded() &&
            self.lon_rounded() == other.lon_rounded()
    }
}

impl Hash for Coordinates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lat_rounded().hash(state);
        self.lon_rounded().hash(state);
    }
}

impl serde::ser::Serialize for Coordinates {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer {
        let mut state =
            serializer.serialize_struct("Coordinates", 2)?;
        state.serialize_field("lat", &self.lat())?;
        state.serialize_field("lon", &self.lon())?;
        state.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for Coordinates {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        enum Field { Lat, Lon }
        ;

        impl<'de> serde::de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`lat` or `lon`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E> where E: de::Error {
                        match value {
                            "lat" => Ok(Field::Lat),
                            "lon" => Ok(Field::Lon),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct CoordinatesVisitor;

        impl<'de> Visitor<'de> for CoordinatesVisitor {
            type Value = Coordinates;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Coordinates")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Coordinates, V::Error> where V: SeqAccess<'de> {
                let lat = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let lon = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Coordinates::from(lat, lon))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Coordinates, V::Error> where V: MapAccess<'de> {
                let mut lat = None;
                let mut lon = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Lat => {
                            if lat.is_some() {
                                return Err(de::Error::duplicate_field("lat"));
                            }
                            lat = Some(map.next_value()?);
                        }
                        Field::Lon => {
                            if lon.is_some() {
                                return Err(de::Error::duplicate_field("lon"));
                            }
                            lon = Some(map.next_value()?);
                        }
                    }
                }
                let lat = lat.ok_or_else(|| de::Error::missing_field("lat"))?;
                let lon = lon.ok_or_else(|| de::Error::missing_field("lon"))?;
                Ok(Coordinates::from(lat, lon))
            }
        }

        const FIELDS: &[&str] = &["lat", "lon"];
        deserializer.deserialize_struct("Coordinates", FIELDS, CoordinatesVisitor)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Highway {
    Motorway,
    Trunk,
    Primary,
    Secondary,
    Tertiary,
    Unclassified,
    Residential,

    MotorwayLink,
    TrunkLink,
    PrimaryLink,
    SecondaryLink,
    TertiaryLink,

    LivingStreet,
    Service,
    Pedestrian,
    Track,
    Road,

    Footway,
    Steps,
    Path,

    Cycleway,
}

impl Highway {
    pub fn from(way: &Way) -> Option<Self> {
        let tag = way.tags.get("highway")?;
        Self::from_str(tag).ok()
    }

    pub fn default_speed(self) -> Option<Kmh> {
        let speed = match self {
            Self::Motorway => 120,
            Self::Trunk => 120,
            Self::Primary => 100,
            Self::Secondary => 100,
            Self::Tertiary => 100,
            Self::Unclassified => 50,
            Self::Residential => 30,
            Self::MotorwayLink => 60,
            Self::TrunkLink => 60,
            Self::PrimaryLink => 50,
            Self::SecondaryLink => 50,
            Self::TertiaryLink => 50,
            Self::LivingStreet => 5,
            Self::Service => 30,
            _ => 30
        };
        Some(Kmh::new(speed))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Kmh {
    pub speed: u8
}

impl Kmh {
    pub fn new(speed: u8) -> Self {
        Self { speed }
    }

    pub fn from(way: &Way) -> Option<Self> {
        let tag = way.tags.get("maxspeed")?;

        if let Ok(speed) = tag.parse::<u8>() {
            Some(Self::new(speed))
        } else {
            let speed: Vec<&str> = tag.split(' ').collect();
            if *speed.get(1)? == "mph" {
                let mph = speed.get(0)?
                    .parse::<u8>().ok()?;
                let kmh = mph as f32 * 1.609_344;
                return Some(Self::new(kmh as u8));
            }
            None
        }
    }

    pub fn time(self, distance: u32) -> u32 {
        let ms = self.speed as f32 / 3.6;
        (distance as f32 / ms).round() as u32
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Surface {
    Paved,
    Unpaved,
    Asphalt,
    Concrete,
    PavingStones,
    Sett,
    Cobblestone,
    Metal,
    Wood,
    Compacted,
    FineGravel,
    Gravel,
    Pebblestone,
    Plastic,
    GrassPaver,
    Grass,
    Dirt,
    Earth,
    Mud,
    Sand,
    Ground,
}

impl Surface {
    pub fn from(way: &Way) -> Option<Self> {
        let tag = way.tags.get("surface")?;
        Self::from_str(tag).ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::osm::Kmh;

    #[test]
    fn time() {
        assert_eq!(14, Kmh::new(50).time(200));
        assert_eq!(36, Kmh::new(20).time(200));
        assert_eq!(144, Kmh::new(5).time(200));
    }
}
