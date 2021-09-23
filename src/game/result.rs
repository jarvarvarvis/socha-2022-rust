use crate::util::error::Error;
use crate::xml::data::server::data::Data;
use crate::xml::data::{conversion::FromDeserializable, enums::PlayerTeam};

#[derive(Debug)]
pub struct GameResult {
    pub winner_team: PlayerTeam
}

impl FromDeserializable<'_, Data> for GameResult {
    fn from_deserializable(serializable: &Data) -> Result<Self, Error> {
        let serializable_winner = &serializable.winner;
        let serializable_winner_ref = serializable_winner.as_ref();
        let winner = serializable_winner_ref.unwrap();
        let winner_team = winner.team.clone();
        
        Ok(Self {
            winner_team
        })
    }
}
