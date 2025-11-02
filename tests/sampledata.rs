// pub struct Plant {
//     id: u64,
//     created: SystemTime,
//     modified: SystemTime,
//     latin_name: String,
//     common_names: Vec<String>,
//     description: String,
//     technical_description: String,
//     uri: String,
//     imageuri: String,
//     price_per_oz: f32,
//     seeds_per_oz: u32,
//     germination_codes: Vec<String>,
//     lifecyle: Lifecycle,
//     light:  Vec<String>,
//     soil_moisture: Vec<String>,
//     height: Dim,
//     width: Dim,
//     flowering_months: Vec<u8>,
//     flower_color: Vec<String>,
//     features: Vec<String>,
//     zones: Vec<u8>,
// }


let sample_plant = Plant {
  id: 987654321,
  created: SystemTime::now(),
  modified: SystemTime::now(),
  common_name: vec!["common milkweed".to_string()],
  latin_name: "asclepias syriaca".to_string(),
  synonyms: vec![
    "silky swallow-wort".to_string(),
    "wild cotton".to_string(),
    "silkweed".to_string(),
    "herbaceous milkweed".to_string(),
  ],
  description: "The large flower can vary in color from nearly white to deep pink-purple. The fragrance is very delicate and pleasing and numerous native pollinators will benefit during its long bloom time.  Common Milkweed looks similar to Prairie Milkweed (Asclepias sullivantii) and Showy Milkweed (Asclepias speciosa). Monarch butterflies lay their eggs exclusively on Milkweed plants, making them the sole food source for their larvae.  Once found in abundance in nearly every farm field, ditch, and disturbed site, Common Milkweed numbers have been in dramatic decline in recent years, due in part to suburban development and the increased efficiency of herbicides used in conjunction with herbicide-tolerant, genetically modified row crops. It spreads readily by seed and underground rhizomes and its taproot can withstand drought.  Common Milkweed is one of the easiest and fastest to establish of the Milkweeds and planting more, even in small urban pockets, can provide personal satisfaction while helping to counter increasing threats to our Monarch butterfly population.  Plant seeds late fall or early spring, no more than 1/4\" deep, on a fairly weed-free site. Collecting and cleaning your own Common Milkweed seed pods is easy.  Watch our VIDEO! Potted plants (3-packs) ship when all plants are well-rooted and transit-ready, early May through June.".to_string(),
  height: Dim::Value(36.0),
  width: Dim::MinMaxmin { min: 24.0, max: 36.0, },
  price_per_oz: 8.15,
  seeds_per_oz: 4000,
  germination_codes: [ GerminationCode::C30, ],
  light: [ light::Full, light::Partial, ],
  soil_moisture: [
    Moisture::MediumWet,
    Moisture::Medium,
    Moisture::MediumDry,
    Moisture::Dry,
  ],
  flowering_months: [
    chrono::Month::June,
    chrono::Month::July,
    chrono::Month::August,
  ],
  flower_color: [
    flower_color::Pink,
  ],
  features: [
    "Pollinator Favorite: butterflies, bees and birds".to_string(),
    "Deer Resistant".to_string(),
  ],
  zones: [
    3,
    4,
    5,
    6,
    7,
    8
  ],
  ..Default::default()
};