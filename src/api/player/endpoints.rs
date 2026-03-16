#![allow(dead_code)]

use crate::api::client::ApiClient;
use crate::api::error::ApiError;
use crate::api::player::{PlayerClubs, PlayerGameArchives, PlayerGameMonthlyArchives, PlayerGames, PlayerGamesToMove, PlayerProfile, PlayerStats, PlayerTournaments, TitledPlayers};

impl ApiClient {
    pub async fn get_player_profile(&self, username: &str) -> Result<PlayerProfile, ApiError> {
        self.get_json(&format!("player/{username}")).await
    }

    pub async fn get_player_stats(&self, username: &str) -> Result<PlayerStats, ApiError> {
        self.get_json(&format!("player/{username}/stats")).await
    }

    pub async fn get_player_games(&self, username: &str) -> Result<PlayerGames, ApiError> {
        self.get_json(&format!("player/{username}/games")).await
    }

    pub async fn get_player_games_to_move(&self, username: &str) -> Result<PlayerGamesToMove, ApiError> {
        self.get_json(&format!("player/{username}/games/to-move")).await
    }

    pub async fn get_player_game_archives(&self, username: &str) -> Result<PlayerGameArchives, ApiError> {
        self.get_json(&format!("player/{username}/games/archives")).await
    }

    pub async fn get_player_game_monthly_archives(&self, username: &str, year: u16, month: u8) -> Result<PlayerGameMonthlyArchives, ApiError> {
        self.get_json_debug(&format!("player/{username}/games/{year}/{month:0>2}")).await
    }

    pub async fn get_player_game_archive_pgn(&self, username: &str, year: u16, month: u8) -> Result<String, ApiError> {
        self.get_text(&format!("player/{username}/games/{year}/{month:0>2}/pgn")).await
    }

    pub async fn get_player_clubs(&self, username: &str) -> Result<PlayerClubs, ApiError> {
        self.get_json(&format!("player/{username}/clubs")).await
    }

    pub async fn get_player_tournaments(&self, username: &str) -> Result<PlayerTournaments, ApiError> {
        self.get_json(&format!("player/{username}/tournaments")).await
    }

    pub async fn get_titled_players(&self) -> Result<TitledPlayers, ApiError> {
        self.get_json_debug("titled/GM").await
    }

}