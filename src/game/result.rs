use crate::util::error::Error;
use crate::xml::data::server::data::Data;
use crate::xml::data::{conversion::FromDeserializable, enums::PlayerTeam};

#[derive(Debug)]
pub struct GameResult {
    pub winner_team: Option<PlayerTeam>,
}

impl FromDeserializable<'_, Data> for GameResult {
    fn from_deserializable(serializable: &Data) -> Result<Self, Error> {
        let winner_team = serializable
            .winner
            .as_ref()
            .map(|winner| winner.team.clone()
        );

        Ok(Self { winner_team })
    }
}
