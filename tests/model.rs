use lol_game_client_api::model::{AllGameData, Events};

#[test]
fn test_events() {
    let data = include_str!("api_response/liveclientdata_events.json");
    let _: Events = serde_json::from_str(data).unwrap();
}

#[test]
fn test_all_game_data() {
    let data = include_str!("api_response/liveclientdata_sample.json");
    let _: AllGameData = serde_json::from_str(data).unwrap();
}