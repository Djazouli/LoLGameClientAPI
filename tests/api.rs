//! This tests can only run if the game is running, it checks the communication with the API.

use lol_game_client_api::api::GameClient;
use tokio;


fn get_game_client() -> GameClient {
    GameClient::new()
}

#[tokio::test]
async fn active_player() {
    let _ = get_game_client().active_player().await.unwrap();
}

#[tokio::test]
async fn active_player_name() {
    let _ = get_game_client().active_player_name().await.unwrap();
}

#[tokio::test]
async fn active_player_abilities() {
    let _ = get_game_client().active_player_abilities().await.unwrap();
}

#[tokio::test]
async fn active_player_runes() {
    let _ = get_game_client().active_player_runes().await.unwrap();
}

#[tokio::test]
async fn player_list() {
    let _ = get_game_client().player_list().await.unwrap();
}

#[tokio::test]
async fn event_data() {
    let _ = get_game_client().event_data().await.unwrap();
}

#[tokio::test]
async fn game_stats() {
    let _ = get_game_client().game_stats().await.unwrap();
}