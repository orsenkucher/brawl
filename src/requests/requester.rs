use crate::payloads::{GetBattlelog, GetPlayer};
use crate::requests::Request;

/// Methods for building requests.
pub trait Requester {
    type Err: std::error::Error + Send;

    type GetBattlelog: Request<Payload = GetBattlelog, Err = Self::Err>;

    fn get_battlelog(&self, tag: String) -> Self::GetBattlelog;

    type GetPlayer: Request<Payload = GetPlayer, Err = Self::Err>;

    fn get_player(&self, tag: String) -> Self::GetPlayer;
}
