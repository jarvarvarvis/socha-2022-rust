extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

use crate::xml::enums::PlayerTeam;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Aggregation {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RelevantForRanking {
    #[serde(rename = "$value")]
    pub is_relevant_for_ranking: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Fragment {
    pub name: String,

    pub aggregation: Aggregation,
    #[serde(rename = "relevantForRanking")]
    pub relevant_for_ranking: RelevantForRanking,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Definition {
    #[serde(rename = "fragment", default)]
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Player {
    pub team: PlayerTeam,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Part {
    #[serde(rename = "$value")]
    pub value: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Score {
    pub cause: String,
    pub reason: String,
    #[serde(rename = "part", default)]
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ScoresEntry {
    pub player: Player,
    pub score: Score,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Scores {
    #[serde(rename = "entry", default)]
    pub entries: Vec<ScoresEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Winner {
    pub team: PlayerTeam,
}
