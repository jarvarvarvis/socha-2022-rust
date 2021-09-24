use crate::util::error::Error;
use crate::xml::server::data::Data;
use crate::xml::{conversion::FromDeserializable, enums::PlayerTeam};

#[derive(Debug)]
pub struct GameResult {
    pub winner_team: Option<PlayerTeam>,
}

impl FromDeserializable<'_, Data> for GameResult {
    fn from_deserializable(deserializable: &Data) -> Result<Self, Error> {
        let winner_team = deserializable
            .winner
            .as_ref()
            .map(|winner| winner.team.clone());

        Ok(Self { winner_team })
    }
}
