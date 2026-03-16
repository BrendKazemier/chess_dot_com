#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClubProfile {
    #[serde(rename="@id")]
    pub id: String,
    pub name: String,
    pub club_id: u32,
    pub country: String,
    pub average_daily_rating: u16,
    pub members_count: u32,
    pub created: u64,
    pub last_activity: u64,
    pub admin: Vec<String>,
    pub visibility: String,
    pub join_request: String,
    pub icon: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClubMembers {
    pub weekly: Vec<Member>,
    pub monthly: Vec<Member>,
    pub all_time: Vec<Member>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub username: String,
    pub joined: u64,
}