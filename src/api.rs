//! Module containing all the necessary code to request `https://127.0.0.1:2999/liveclientdata/` endpoints.
//! The official documentation can be found at [https://developer.riotgames.com/docs/lol#game-client-api]

use reqwest::{Certificate, IntoUrl};
use serde::Deserialize;
use crate::model::{Abilities, ActivePlayer, AllGameData, Events, FullRunes, GameData, Player};
use thiserror::Error;
use crate::api;

pub struct GameClient {
    client: reqwest::Client
}

/// Get the root certificate that is used to sign the SSL certificates.
/// You need to trust this certificate to be able to make requests to the API.
#[cold]
pub fn get_riot_root_certificate() -> Certificate {
    Certificate::from_pem(include_bytes!("riotgames.cer")).unwrap()
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Failed to query the API. Is the game running ?")]
    Reqwest(#[from] reqwest::Error), // An error of this type may suggests that the API specs as been updated and the crate is out-of-date. Please fill an issue !
}

impl GameClient {
    /// Create a new `GameClient`using a `reqwest::Client`.
    /// As Riot Games self-sign their SSL certificates, it would be great to trust them in the `reqwest::Client`.
    pub fn new(client: reqwest::Client) -> Self {
        GameClient {
            client
        }
    }

    /// Query the endpoint and automagically deserialize the json into the desired type
    async fn get_data<T: for<'de> Deserialize<'de>, U: IntoUrl>(&self, endpoint: U) -> Result<T, QueryError> {
        let data = self.client.get(endpoint).send().await?.json::<T>().await?;
        Ok(data)
    }
}

/// All the implemented endpoints
impl GameClient {
    /// Get all available data.
    pub async fn all_game_data(&self) -> Result<AllGameData, QueryError> {
        self.get_data(api!("allgamedata")).await
    }

    /// Get all data about the active player.
    pub async fn active_player(&self) -> Result<ActivePlayer, QueryError> {
        self.get_data(api!("activeplayer")).await
    }

    /// Returns the player name.
    pub async fn active_player_name(&self) -> Result<String, QueryError> {
        self.get_data(api!("activeplayername")).await
    }

    /// Get Abilities for the active player.
    pub async fn active_player_abilities(&self) -> Result<Abilities, QueryError> {
        self.get_data(api!("activeplayerabilities")).await
    }

    /// Retrieve the full list of runes for the active player.
    pub async fn active_player_runes(&self) -> Result<FullRunes, QueryError> {
        self.get_data(api!("activeplayerrunes")).await
    }

    /// Retrieve the list of heroes in the game and their stats.
    pub async fn player_list(&self) -> Result<Vec<Player>, QueryError> {
        self.get_data(api!("playerlist")).await
    }

    // TODO: Infos specific to a player.

    /// Get a list of events that have occurred in the game.
    pub async fn event_data(&self) -> Result<Events, QueryError> {
        self.get_data(api!("eventdata")).await
    }

    /// Basic data about the game.
    pub async fn game_stats(&self) -> Result<GameData, QueryError> {
        self.get_data(api!("gamestats")).await
    }

}

#[macro_export]
macro_rules! api {
    ($endpoint:expr) => {
        concat!("https://127.0.0.1:2999/liveclientdata/", $endpoint)
    }
}