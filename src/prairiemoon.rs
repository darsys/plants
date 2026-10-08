mod plant;
use serde::{Deserialize, Serialize};

pub struct prairiemoon_plant {
  #[serde(alias = "name")
  latin_name: String
  #[serde(alias = "commonname")]
  common_name: String,
  description: String,
  prices: Vec<price>,
  #[serde(rename="seeds/packet"))]
  seedsperpacket: u32,
  #[serde(rename="seeds/ounce"))]
  seedsperounce: u32,
  germination: Vedc<String>,
  "light": [
    "Full",
    "Partial"
  ],
  "moisture": [

}

pub struct price {
      quantity: u16,
      unit: String,
      price: f32
}

  "seeds/packet": 300,
  "seeds/ounce": 6600,  "germination": [
    "A"
  ],
  "light": [
    "Full",
    "Partial"
  ],
  "moisture": [
    "Medium-Wet",
    "Medium",
    "Medium-Dry"
  ],
  "height": 48,
  "blooms": [
    "July",
    "August",
    "September"
  ],
  "color": [
    "Purple"
  ],
  "features": [
    "Pollinator Favorite: butterflies, bees and birds",
    "Deer Resistant (Our experiences here in the Upper Midwest may vary in other regions; deer can respond differently to local conditions or seasonal variations.)",
    "Highly recommended for home landscaping"
  ],
  "zones": [
    4,
    5,
    6,
    7,
    8
  ],
  "spacing": {
    "min": 18,
    "max": 24
  },
  "sku": "ECH08F"
}
