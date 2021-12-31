This crate aims to provide a Rust interface with the League of Legends Game Client API. The official doc can be found [here](https://developer.riotgames.com/docs/lol#game-client-api).


The Game Client API is an API that is exposed by your local League of Legends Client,
and should not be mistaken for the LoL API, for which there is already [a great crate](https://github.com/MingweiSamuel/Riven).
The API allows you to get some informations about the ongoing game.

# Content of the crate

There are currently three modules: 
- `api` which contains the `GameClient` and all it's methods that calls the API.
- `model` which contains all the structures for the data that will be returned by the API.
- `event_listener` which contains the definition of a trait to create structures that you can pass to `start_listener` to run a task that will periodically check for new `Events` and trigger callbacks accordingly. (you can see an example in `examples/basic_listener`)

# How to use it
```rust
use lol_game_client_api::api::{GameClient, get_riot_root_certificate};
use tokio;

#[tokio::main]
async fn main() {
    let client = GameClient::new();
    let active_player = client.active_player().await.unwrap();
    
    println!("Stats Runes: {:?}", active_player.full_runes.stat_runes)
}
```

A more complete example can be found in the repo [LoLCongratz](https://github.com/Djazouli/LoLCongratz). This is 
a small binary that plays a sound in a virtual audio cable each time a teammate makes a kill.

## TODO
Currently, some variants are missing from enums (see `model::DragonType` for example).  
Also, some endpoints are not working at all (
all the endpoints that require the `summonerName` as a `GET` param for example).  
But all the structs to deserialize the response already exists, so this should not be too much work.

## Disclaimer

I created this crate a morning, for a personnal project, so there may be some missing features,
or it may not be up-to-date with the latest LoL client. In which case, I'd be delighted to see a contribution (being an issue or a PR) !
