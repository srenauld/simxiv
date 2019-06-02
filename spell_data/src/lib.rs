#[macro_use] extern crate serde_derive;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::path::Path;
use std::error::Error;

use serde::de::{Deserialize, Deserializer, Unexpected};
use serde_repr::{Deserialize_repr};

pub type ActionId = u32;

fn deserialize_melee<'de, D: Deserializer<'de>>(des: D) -> Result<Range, D::Error> {
    let range = i32::deserialize(des)?;
    if range == -1 {
        Ok(Range::Melee)
    } else if range < 0 {
        Err(serde::de::Error::invalid_value(
                Unexpected::Signed(range as i64),
                &"A non-negative number or -1"
        ))
    } else {
        Ok(Range::Ranged(range as u32))
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Range {
    Ranged(u32),
    Melee,
}

#[derive(Deserialize_repr, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum KnownCost {
    Mana = 3,
    Tact = 5,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum CostType {
    Known(KnownCost),
    Unknown(u8),
}

fn coinach_bool<'de, D: Deserializer<'de>>(des: D) -> Result<bool, D::Error> {
    match String::deserialize(des)?.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        other => Err(serde::de::Error::invalid_value(
                Unexpected::Str(other),
                &"True or False"
        ))
    }
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all="PascalCase")]
pub struct RawAction {
    #[serde(rename="#")]
    pub id: ActionId,
    pub name: String,
    #[serde(deserialize_with="deserialize_melee")]
    pub range: Range,
    #[serde(deserialize_with="coinach_bool")]
    pub can_target_friendly: bool,
    #[serde(deserialize_with="coinach_bool")]
    pub can_target_self: bool,
    #[serde(deserialize_with="coinach_bool")]
    pub can_target_hostile: bool,
    #[serde(rename="Cost{Type}")]
    pub cost_type: CostType,
    pub cost: u32,
    #[serde(rename="Cast<100ms>")]
    pub cast: u32,
    #[serde(rename="Recast<100ms>")]
    pub recast: u32,
}

pub fn load_actions<P: AsRef<Path>>(path: P) -> Result<HashMap<ActionId, RawAction>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;

    rdr.into_deserialize::<RawAction>().map(|res| res.map(|action| (action.id, action)).map_err(|e| e.into())).collect()
}
