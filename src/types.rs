use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    club: PlayerClub,
    is_qualified_from_championship_challenge: bool,
    #[serde(rename = "3vs3Victories")]
    victories_3v3: i32,
    name: String,
    tag: String,
    trophies: i32,
    exp_level: i32,
    exp_points: i32,
    name_color: String,
    brawlers: Vec<Brawler>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerClub {
    tag: String,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brawler {
    id: i32,
    // rank: i32,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Battles {
    items: Vec<Battle>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battle {
    battle: BattleInner,
    battle_time: String,
    event: Event,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleInner {
    mode: EventMode,
    r#type: String, // TODO: enum
    result: Option<String>,
    duration: Option<i32>,
    trophy_change: Option<i32>,
    star_player: Option<StarPlayer>,
    teams: Option<Vec<Team>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StarPlayer {
    tag: String,
    name: String,
    brawler: Brawler,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Team(Vec<StarPlayer>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    // #[serde(flatten)]
    mode: EventMode,
    map: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventMode {
    SoloShowdown,
    DuoShowdown,
    Heist,
    Bounty,
    Siege,
    GemGrab,
    BrawlBall,
    BigGame,
    BossFight,
    RoboRumble,
    Takedown,
    LoneStar,
    PresentPlunder,
    HotZone,
    SuperCityRampage,
    Knockout,
    VolleyBrawl,
    BasketBrawl,
    HoldTheTrophy,
    TrophyThieves,
    Duels,
    Wipeout,
    Payload,
    Invasion,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battlelog_deserialize() {
        let json = include_str!("battlelog.json");
        let battles: Battles = serde_json::from_str(json).unwrap();
        assert_eq!(battles.items.len(), 25);
    }
}
