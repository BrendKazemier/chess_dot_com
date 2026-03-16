#![cfg(feature = "live-tests")]

use chess_dot_com::api::client::ApiClient;

#[tokio::test]
async fn test_live_get_club_profile() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let club_profile = api.get_club_profile("pro-chess-league-late-qualifier").await;

    assert!(club_profile.is_ok())
}

#[tokio::test]
async fn test_live_get_club_members() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let club_members = api.get_club_members("pro-chess-league-late-qualifier").await;

    assert!(club_members.is_ok())
}