#![allow(dead_code)]

use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerProfile {
    pub avatar: String,
    pub player_id: u64,
    #[serde(rename="@id")]
    pub id: String,
    pub url: String,
    pub name: String,
    pub username: String,
    pub followers: u64,
    pub country: String,
    pub location: String,
    pub last_online: u64,
    pub joined: u64,
    pub status: String,
    pub is_streamer: bool,
    pub verified: bool,
    pub league: String,
    pub streaming_platforms: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerStats {
    pub chess_daily: ChessStats,
    pub chess960_daily: Option<ChessStats>,
    pub chess_rapid: Option<ChessStats>,
    pub chess_bullet: Option<ChessStats>,
    pub chess_blitz: Option<ChessStats>,
    pub fide: Option<u16>,
    pub tactics: Tactics,
    pub puzzle_rush: PuzzleRush,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChessStats {
    pub last: Rating,
    pub best: Option<Rating>,
    pub record: Record,
    pub tournament: Option<TournamentStats>
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub win: u32,
    pub loss: u32,
    pub draw: u32,
    pub tournament: Option<TournamentStats>,
    pub time_per_move: Option<u32>,
    pub timeout_percent: Option<u8>,
}


#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TournamentStats {
    pub points:u16,
    pub withdraw: u16,
    pub count: u16,
    pub highest_finish: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tactics {
    pub highest: Rating,
    pub lowest: Rating,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rating {
    pub rating: u16,
    pub date: u64,
    pub game: Option<String>,
    pub rd: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PuzzleRush {
    pub best: PuzzleRushStats,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PuzzleRushStats {
    pub total_attempts: u32,
    pub score: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerGames {
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub url: String,
    pub move_by: u64,
    pub pgn: String,
    pub time_control: String,
    pub last_activity: u64,
    pub rated: bool,
    pub turn: String,
    pub fen: String,
    pub start_time: u64,
    pub time_class: String,
    pub rules: String,
    pub white: String,
    pub black: String,
    pub tournament: Option<String>,
    #[serde(rename="match")]
    pub r#match: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerGamesToMove {
    pub games: Vec<GameToMove>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameToMove {
    pub url: String,
    pub move_by: u64,
    pub last_activity: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerGameArchives {
    archives: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerGameMonthlyArchives {
    games: Vec<PlayerGameArchive>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerGameArchive {
    pub url: String,
    pub pgn: String,
    pub time_control: String,
    pub end_time: u64,
    pub rated: bool,
    pub accuracies: Option<Accuracies>,
    pub tcn: String,
    pub uuid: String,
    pub initial_setup: String,
    pub fen: String,
    pub start_time: Option<u64>,
    pub time_class: String,
    pub rules: String,
    pub white: GameOutcome,
    pub black: GameOutcome,
    pub eco: Option<String>,
    pub tournament: Option<String>,
    pub r#match: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Accuracies {
    pub white: f32,
    pub black: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameOutcome {
    pub rating: u16,
    pub result: String,
    #[serde(rename="@id")]
    pub id: String,
    pub username: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerClubs {
    pub clubs: Vec<Club>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    #[serde(rename="@id")]
    pub id: String,
    pub name: String,
    pub last_activity: u64,
    pub icon: String,
    pub url: String,
    pub joined: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerTournaments {
    pub finished: Vec<Tournament>,
    pub in_progress: Vec<Tournament>,
    pub registered: Vec<Tournament>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tournament {
    pub url: String,
    #[serde(rename="@id")]
    pub id: String,
    pub wins: u8,
    pub losses: u8,
    pub draws: u8,
    pub points_awarded: Option<u16>,
    pub placement: Option<u16>,
    pub status: String,
    pub total_players: u16,
    pub time_class: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TitledPlayers {
    pub players: Vec<String>,
}