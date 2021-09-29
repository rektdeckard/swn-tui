use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    planet: HashMap<String, Planet>,
    sector: HashMap<String, Sector>,
    system: HashMap<String, System>,
    black_hole: HashMap<String, System>,
}

impl World {
    pub fn sector(&self) -> &Sector {
        self.sector
            .values()
            .nth(0)
            .expect("The data contained no Sector")
    }

    pub fn planets(&self) -> Vec<&Planet> {
        self.planet.values().collect()
    }

    pub fn systems(&self) -> Vec<&System> {
        self.system.values().collect()
    }

    pub fn black_holes(&self) -> Vec<&System> {
        self.black_hole.values().collect()
    }

    pub fn parent_planet(&self, id: &str) -> Option<&Planet> {
        self.planet.get(id)
    }

    pub fn child_planets(&self, system: &System) -> Option<Vec<&Planet>> {
        let maybe_id = self
            .system
            .iter()
            .find_map(|(key, value)| if value == system { Some(key) } else { None });

        if let Some(id) = maybe_id {
            Some(
                self.planet
                    .values()
                    .filter(|&p| &p.parent == id && p.parent_entity == "system")
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn parent_system(&self, id: &str) -> &System {
        self.system.get(id).expect("Bad ID for planet")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sector {
    pub attributes: Option<Value>,
    pub columns: u8,
    pub rows: u8,
    pub created: Option<String>,
    pub creator: Option<String>,
    pub name: String,
    pub updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub attributes: Option<Value>,
    pub created: Option<String>,
    pub creator: Option<String>,
    pub is_hidden: bool,
    pub name: String,
    pub parent: String,
    pub parent_entity: String,
    pub updated: Option<String>,
    pub x: u8,
    pub y: u8,
}

impl System {
    pub fn hex(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Planet {
    pub attributes: Option<Value>,
    pub created: Option<String>,
    pub creator: Option<String>,
    pub is_hidden: bool,
    pub name: String,
    pub parent: String,
    pub parent_entity: String,
    pub updated: Option<String>,
}
