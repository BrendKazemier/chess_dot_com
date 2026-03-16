// #![cfg(feature = "live-tests")]

use chess_dot_com::api::client::ApiClient;

#[tokio::test]
async fn test_live_get_player_profile() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_profile = api.get_player_profile("erik").await;

    assert!(player_profile.is_ok())
}

#[tokio::test]
async fn test_live_get_player_stats() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_stats = api.get_player_stats("erik").await;

    assert!(player_stats.is_ok())
}

#[tokio::test]
async fn test_live_get_player_games() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_games = api.get_player_games("erik").await;

    assert!(player_games.is_ok())
}

#[tokio::test]
async fn test_live_get_player_games_to_move() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_games_to_move = api.get_player_games_to_move("erik").await;

    assert!(player_games_to_move.is_ok())
}

#[tokio::test]
async fn test_live_get_player_game_archives() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_games_to_move = api.get_player_game_archives("erik").await;

    assert!(player_games_to_move.is_ok())
}

#[tokio::test]
async fn test_live_get_player_game_monthly_archives() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_game_monthly_archives = api.get_player_game_monthly_archives("erik", 2026, 1).await;

    assert!(player_game_monthly_archives.is_ok())
}

#[tokio::test]
async fn test_live_get_player_game_archive_pgn() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_game_archive_pgn = api.get_player_game_archive_pgn("erik", 2026, 1).await;

    assert!(player_game_archive_pgn.is_ok())
}

#[tokio::test]
async fn test_live_get_player_clubs() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_clubs = api.get_player_clubs("erik").await;

    assert!(player_clubs.is_ok())
}

#[tokio::test]
async fn test_live_get_player_tournaments() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let player_tournaments = api.get_player_tournaments("erik").await;

    assert!(player_tournaments.is_ok())
}

#[tokio::test]
async fn test_live_get_titled_players() {
    let api = ApiClient::new("https://api.chess.com/pub").unwrap();

    let titled_players = api.get_titled_players().await;

    assert!(titled_players.is_ok())
}