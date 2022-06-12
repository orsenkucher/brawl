use serde::Serialize;

use crate::{
    requests::Payload,
    types::{Battles, Player},
};

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
pub struct GetPlayer {
    pub tag: String,
}

impl GetPlayer {
    pub fn new(tag: String) -> Self {
        GetPlayer { tag }
    }
}

impl Payload for GetPlayer {
    type Output = Player;

    fn name(&self) -> String {
        format!("players/{}", self.tag)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
pub struct GetBattlelog {
    pub tag: String,
}

impl GetBattlelog {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }
}

impl Payload for GetBattlelog {
    type Output = Battles;

    fn name(&self) -> String {
        format!("players/{}/battlelog", self.tag)
    }
}
