use crate::event_manager::EventManager;
use std::time::Duration;

mod event_manager;

#[tokio::main]
async fn main() {
    env_logger::init();

    let event_listener = EventManager::new();

    // Start a task that will check every second for new events
    lol_game_client_api::start_listener(event_listener, Duration::from_secs(1));

    loop {
        tokio::time::sleep(Duration::from_secs(180)).await; // Sleep whatever, this is just to avoid wasting CPU cycles
    }
}
