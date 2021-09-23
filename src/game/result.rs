use crate::xml::data::server::data::Data;
use crate::xml::data::{conversion::FromSerializable, enums::PlayerTeam};

#[derive(Debug)]
pub struct Result {
    pub points: (i32, i32)
}

impl Result {
    pub fn get_points_for_team(&self, team: PlayerTeam) -> i32 {
        match team {
            PlayerTeam::One => self.points.0,
            PlayerTeam::Two => self.points.1,
        }
    }
}

impl FromSerializable<'_, Data> for Result {
    fn from_serializable(serializable: &Data) -> Self {
        todo!()
    }
}
