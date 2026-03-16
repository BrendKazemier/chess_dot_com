#![allow(dead_code)]

use crate::api::client::ApiClient;
use crate::api::error::ApiError;
use crate::api::tournament::{TournamentInformation, TournamentRoundGroupInformation, TournamentRoundInformation};

impl ApiClient {
    pub async fn get_tournament_information(&self, tournament: &str) -> Result<TournamentInformation, ApiError> {
        self.get_json(&format!("tournament/{tournament}")).await
    }
    
    pub async fn get_tournament_round_information(&self, tournament: &str, round: u8) -> Result<TournamentRoundInformation, ApiError> {
        self.get_json(&format!("tournament/{tournament}/{round}")).await
    }
    
    pub async fn get_tournament_round_group_information(&self, tournament: &str, round: u8, group: u8) -> Result<TournamentRoundGroupInformation, ApiError> {
        self.get_json_debug(&format!("tournament/{tournament}/{round}/{group}")).await
    }
}