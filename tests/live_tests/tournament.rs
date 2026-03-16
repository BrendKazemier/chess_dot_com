#![cfg(feature = "live-tests")]

use chess_dot_com::api::client::ApiClient;

#[tokio::test]
async fn test_live_get_tournament_information() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let tournament = api.get_tournament_information("-33rd-chesscom-quick-knockouts-1401-1600").await;

    assert!(tournament.is_ok())
}

#[tokio::test]
async fn test_live_get_tournament_round_information() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let tournament = api.get_tournament_round_information("-33rd-chesscom-quick-knockouts-1401-1600", 1).await;

    assert!(tournament.is_ok())
}

#[tokio::test]
async fn test_live_get_tournament_round_group_information() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let tournament = api.get_tournament_round_group_information("-33rd-chesscom-quick-knockouts-1401-1600", 1, 1).await;

    assert!(tournament.is_ok())
}