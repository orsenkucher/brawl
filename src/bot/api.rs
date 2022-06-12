use crate::{
    payloads,
    requests::{JsonRequest, Requester},
};

use super::Bot;

impl Requester for Bot {
    type Err = crate::errors::RequestError;

    type GetBattlelog = JsonRequest<payloads::GetBattlelog>;

    fn get_battlelog(&self, tag: String) -> Self::GetBattlelog {
        Self::GetBattlelog::new(self.clone(), payloads::GetBattlelog::new(tag))
    }

    type GetPlayer = JsonRequest<payloads::GetPlayer>;

    fn get_player(&self, tag: String) -> Self::GetPlayer {
        Self::GetPlayer::new(self.clone(), payloads::GetPlayer::new(tag))
    }
}
