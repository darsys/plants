use axum::{
    http::StatusCode,
    Json
};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Plant {
    #[serde(default)]  
    id: u64,
    created: Option<SystemTime>,
    modified: Option<SystemTime>,
    #[serde(alias = "name")]
    latin_name: String,
    #[serde(alias = "commonName")]
    commonname: String, 
    description: Option<String>,
    technical_description: Option<String>,
    uri: Option<String>,
    imageuri: Option<String>,
    price_per_oz: Option<u32>,
    #[serde(alias = "seeds/ounce")]  
    seeds_per_oz: Option<u32>,
    #[serde(alias = "germination")]  
    germination_codes: Option<Vec<Germination>>,
    habit: Option<Habit>,
    lifecyle: Option<Lifecycle>,
    light:  Option<Vec<Light>>,
    #[serde(alias = "moisture")]  
    soil_moisture: Option<Vec<Moisture>>,
    soil_type: Option<Vec<Soil>>,
    height: Option<Dim>,
    spacing: Option<Dim>,
    flowering_months: Option<Vec<Month>>,
    flower_color: Option<Vec<FlowerColor>>,
    features: Option<Vec<String>>,
    zones: Option<Vec<u8>>,
}


// pub struct LatinName {
//     genus: String,
//     specific_epithet: String,
// }


#[derive(Deserialize, Serialize)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
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
  #[serde(alias = "Medium-Wet")] 
  MediumWet,
  Medium,
  #[serde(alias = "Medium-Dry")]
  MediumDry,
  Dry
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
    // Unknown
    Unknown,
    // Easy to germinate
    Easy,
    // Difficult
    Difficult, 
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

pub async fn create_plant_from_json(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Plant>,
) -> (StatusCode, Json<Plant>) {
    // insert your application logic here

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(payload))
}

