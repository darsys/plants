use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use axum::{
    http::StatusCode,
    Json
};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Plant {
    id: u64,
    created: SystemTime,
    modified: SystemTime,
    latin_name: String,
    common_names: Vec<String>,
    description: String,
    technical_description: String,
    uri: String,
    imageuri: String,
    price_per_oz: f32,
    seeds_per_oz: u32,
    germination_codes: Vec<Germination>,
    habit: Habit,
    lifecyle: Lifecycle,
    light:  Vec<Light>,
    soil_moisture: Vec<String>,
    height: Dim,
    width: Dim,
    flowering_months: Vec<chrono::Month>,
    flower_color: Vec<String>,
    features: Vec<String>,
    zones: Vec<u8>,
}

// pub struct LatinName {
//     genus: String,
//     specific_epithet: String,
// }

#[derive(Deserialize, Serialize)]
pub enum Dim {
  Value(f32),
  MinMax { min: f32, max: f32},
}

#[derive(Deserialize, Serialize)]
pub enum Habit {
  ForbHerb,
  Graminoid,
  Nonvascular,
  Lichenous,
  Shrub,
  Tree,
  Vine
}

#[derive(Deserialize, Serialize)]
pub enum Lifecycle {
  Annual,
  Perennial,
  Biennial
}

#[derive(Deserialize, Serialize)]
pub enum Soil {
  Sand,
  Loam,
  Clay
}

#[derive(Deserialize, Serialize)]
pub enum Light {
  Full,
  Partial,
  Shade
}

#[derive(Deserialize, Serialize)]
pub enum Moisture {
  Wet,
  MediumWet,
  Medium,
  MediumDry,
  Dry
}

#[derive(Deserialize, Serialize)]
pub enum Zones {
  Zone1 = 1,
  Zone2 = 2,
  Zone3 = 3,
  Zone4 = 4,
  Zone5 = 5,
  Zone6 = 6,
  Zone7 = 7,
  Zone8 = 8,
  Zone9 = 9
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum Germination {
/// No pretreatment required
    A, /// Cold stratification for 30 days
    B, /// Hot water treatment
    C30, /// Cold stratification for 60 days
    C60, /// Cold stratification for 90 days
    C90, /// Cold stratification for 120 days
    C120, /// Cold stratification for unspecified duration
    D, /// Small seed or Light required, sow on surface
    E, /// Warn moist period then Cold moist period
    F, /// Cold moist period then warn moist period 
    G, /// Cool soil, fall sowing
    H, /// Scarification required
    I, /// Inoculation (rhizobia) required
    J, /// Legume with hulls removed
    K, /// Hemiparasite species requires host plant
    Unknown, /// Empty or question mark
    Easy, /// Easy to germinate
    Difficult, /// Difficult to germinate
}

#[derive(Deserialize, Serialize)]
pub enum FlowerColor { 
  Blue,
  Brown,
  Green,
  Orange,
  Purple,
  Red,
  White,
  Yellow,
  Pink,
}

pub async fn read_plant_from_file<P: AsRef<Path>>(path: P) -> Result<Plant, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Plant`.
    let p = serde_json::from_reader(reader)?;

    // Return the `Plant`.
    Ok(p)
}

pub async fn create_plant(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Plant>,
) -> (StatusCode, Json<Plant>) {
    // insert your application logic here

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(payload))
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }
}



// json sample data 
// {
//   "name": "echinacea purpurea",
//   "commonName": "purple coneflower",
//   "description": "The showy daisy-like flowers of Pale Purple Coneflower bloom in early summer and are a favorite nectar source for butterflies and myriad pollinators, including hummingbirds. Later in summer the large seedheads attract goldfinches and other birds.Echinacea pallida is a highly adaptable plant that is tolerant of drought, heat, humidity and poor soils, but it will not like soils that are too moist with poor drainage. Once established the deep taproot enables a long-lived, very low maintenance plant that is capable of handling hot dry situations with ease. This iconic prairie plant looks its best in a naturalized setting that includes other prairie flowers and grasses, or in a mixed border garden. Echinacea plants were used by Native Americans for medicinal purposes and are still used today in herbal medicine and tea. Echinacea comes from the Greek word echinos meaning hedgehog in reference to the spiny center cone.",
//   "uri": "https://www.prairienursery.com/store/seeds/golden-alexanders-seed-zizia-aurea",
//   "imageuri": "https://www.prairienursery.com/_assets/products/1636/product-2800.jpg",
//   "seeds/ounce": 6600,
//   "prices": [
//     {
//       "quantity": 0.25,
//       "unit": "oz",
//       "price": "$3.00"
//     },
//     {
//       "quantity": 0.5,
//       "unit": "oz",
//       "price": "$5.00"
//     },
//     {
//       "quantity": 1,
//       "unit": "oz",
//       "price": "$8.00"
//     },
//     {
//       "quantity": 1,
//       "unit": "lb",
//       "price": "$120.00"
//     }
//   ],
//   "light": [
//     "Full",
//     "Partial"
//   ],
//   "soil": [
//     "Sand",
//     "Loam",
//     "Clay"
//   ],
//   "moisture": [
//     "Dry",
//     "Medium"
//   ],
//   "benefits": [
//     "Pollinators",
//     "Butterflies",
//     "Birds",
//     "Hummingbirds",
//     "Deer Resistant"
//   ],
//   "height": {
//     "min": 36,
//     "max": 48
//   },
//   "blooms": [
//     "July",
//     "August",
//     "September"
//   ],
//   "zones": [
//     4,
//     5,
//     6,
//     7,
//     8
//   ],
//   "color": [
//     "Purple"
//   ],
//   "spacing": {
//     "min": 12,
//     "max": 12
//   },
//   "root": "Fibrous"
// }