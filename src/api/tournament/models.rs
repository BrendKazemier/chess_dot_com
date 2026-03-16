#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentInformation {
    pub name: String,
    pub url: String,
    pub description: String,
    pub creator: String,
    pub status: String,
    pub finish_time: u64,
    pub settings: TournamentSettings,
    pub players: Vec<TournamentPlayer>,
    pub rounds: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentSettings {
    pub r#type: String,
    pub rules: String,
    pub is_rated: bool,
    pub is_official: bool,
    pub is_invite_only: bool,
    pub min_rating: u16,
    pub max_rating: u16,
    pub initial_group_size: u8,
    pub user_advance_count: u8,
    pub use_tiebreak: bool,
    pub allow_vacation: bool,
    pub winner_places: u8,
    pub registered_user_count: u16,
    pub games_per_opponent: u8,
    pub total_rounds: u8,
    pub concurrent_games_per_opponent: u8,
    pub time_class: String,
    pub time_control: String,
}



#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentPlayer {
    pub username: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentRoundInformation {
    pub groups: Vec<String>,
    pub players: Vec<TournamentRoundPlayer>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentRoundPlayer {
    pub username: String,
    pub is_advancing: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentRoundGroupInformation {
    // Assuming it's strings for now.
    pub fair_play_removals: Vec<String>,
    pub games: Vec<TournamentRoundGroupGame>,
    pub players: Vec<TournamentRoundGroupPlayer>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentRoundGroupGame {
    pub url: String,
    pub pgn: String,
    pub time_control: String,
    pub end_time: u64,
    pub rated: bool,
    pub fen: String,
    pub start_time: u64,
    pub time_class: String,
    pub rules: String,
    pub white: TournamentRoundGroupOutcome,
    pub black: TournamentRoundGroupOutcome,
    pub tournament: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentRoundGroupOutcome {
    pub rating: u16,
    pub result: String,
    #[serde(rename="@id")]
    pub id: String,
    pub username: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentRoundGroupPlayer {
    pub username: String,
    pub points: f32,
    pub is_advancing: bool,
    pub tie_break: f32,
}