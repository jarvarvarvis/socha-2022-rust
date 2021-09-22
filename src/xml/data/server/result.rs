extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

use crate::xml::data::enums::{PlayerTeam};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Aggregation {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RelevantForRanking {
    #[serde(rename = "$value")]
    is_relevant_for_ranking: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Fragment {
    name: String,

    aggregation: Aggregation,
    #[serde(rename = "relevantForRanking")]
    relevant_for_ranking: RelevantForRanking,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Definition {
    #[serde(rename = "fragment", default)]
    fragments: Vec<Fragment>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Player {
    team: PlayerTeam,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Part {
    #[serde(rename = "$value")]
    value: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Score {
    cause : String,
    reason: String,
    #[serde(rename = "part", default)]
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ScoresEntry {
    player: Player,
    score: Score,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Scores {
    #[serde(rename = "entry", default)]
    entries: Vec<ScoresEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Winner {
    team: PlayerTeam,
}
