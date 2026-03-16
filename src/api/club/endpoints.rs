#![allow(dead_code)]

use crate::api::client::ApiClient;
use crate::api::error::ApiError;
use crate::api::club::{ClubMembers, ClubProfile};

impl ApiClient {
    pub async fn get_club_profile(&self, club: &str) -> Result<ClubProfile, ApiError> {
        self.get_json(&format!("club/{club}")).await
    }

    pub async fn get_club_members(&self, club: &str) -> Result<ClubMembers, ApiError> {
        self.get_json(&format!("club/{club}/members")).await
    }
}