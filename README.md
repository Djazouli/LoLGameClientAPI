This crate aims to provide a Rust interface with the League of Legends Game Client API. The official doc can be found here:
https://developer.riotgames.com/docs/lol#game-client-api

The Game Client API is an API that is exposed by your local League of Legends Client,
and should not be mistaken for the LoL API, for which there is already [a great crate](https://github.com/MingweiSamuel/Riv000en).
The API allows you to get some informations about the ongoing game.

# How to use it
```rust
use lol_game_client_api::api::{GameClient, get_riot_root_certificate};
use tokio;

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new().add_root_certificate(get_riot_root_certificate()).build().unwrap();
    let client = GameClient::new(client);
    let active_player = client.active_player().await.unwrap();
    
    println!("Stats Runes: {:?}", active_player.full_runes.stat_runes)
}
```

## Disclaimer

I created this crate a morning, for a personnal project, so there may be some missing features,
or it may not be up-to-date with the latest LoL client. In which case, I'd be delighted to see a contribution (being an issue or a PR) !
