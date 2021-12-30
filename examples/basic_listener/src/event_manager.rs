use lol_game_client_api::api::{GameClient, QueryError};
use lol_game_client_api::async_trait::async_trait;
use lol_game_client_api::event_listener::EventListener;
use lol_game_client_api::model::{Ace, Team};
use thiserror::Error;

pub struct EventManager {
    game_client: GameClient, // Is used to query the team if needed, or other info on the fly
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            game_client: GameClient::new(),
        }
    }

    /// Get the team of the `active_player`
    async fn get_current_team(&self) -> Result<Team, EventManagerError> {
        let active_player = self.game_client.active_player().await?.summoner_name;

        for player in self.game_client.player_list().await? {
            if player.summoner_name == active_player {
                return Ok(player.team);
            }
        }

        Err(EventManagerError::PlayerNotFound)
    }
}

#[derive(Error, Debug)]
pub enum EventManagerError {
    #[error("Game client API error: {}", _0)]
    GameClientApi(#[from] QueryError),
    #[error("Failed to find active player in player list")]
    PlayerNotFound,
}

#[async_trait]
impl EventListener for EventManager {
    type Error = EventManagerError;

    async fn on_ace(&mut self, event: Ace) -> Result<(), Self::Error> {
        if self.get_current_team().await? == event.acing_team {
            println!("Yahou, we aced")
        }

        Ok(())
    }
}
