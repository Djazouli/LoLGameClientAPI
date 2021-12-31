use crate::api::GameClient;
use crate::event_listener::EventListener;
use std::error::Error;
use std::time::Duration;

pub mod api;
pub mod event_listener;
pub mod model;

pub extern crate async_trait; // Re-export async_trait

/// Take an event listener, and start a tasks that will periodically list all new events, and
/// pass them to the listener callbacks.
///
/// This default implementation is fairly naive, and may quickly be obsolete.
/// Not only does it require a strong bound on the Error type, but it also has poor performances.
/// This waits for an event to be fully processed before processing another one. If a processing take some time,
/// the processing of future events will be strongly delayed, which is suboptimal.
pub fn start_listener<L: EventListener + 'static>(mut listener: L, period: Duration) {
    tokio::spawn(async move {
        let client = GameClient::new();

        let mut previous_number_of_events = match client.event_data().await {
            Ok(events) => {
                log::info!("Running the program when the game is already started. {} events already happened.", events.events.len());
                events.events.len()
            }
            Err(_) => {
                log::info!("Game is not running yet");
                0
            }
        };

        loop {
            let events = match client.event_data().await {
                Ok(events) => events.events,
                Err(err) => {
                    log::error!("Failed to get event data: {}", err);
                    tokio::time::sleep(period).await;
                    continue;
                }
            };

            let number_of_events = events.len();

            if number_of_events <= previous_number_of_events {
                // Nothing new...
                log::debug!("Nothing new, still {} events", number_of_events);
                tokio::time::sleep(period).await;
                continue;
            }

            for event in events.as_slice()[previous_number_of_events..]
                .to_vec()
                .into_iter()
            {
                match listener.on_event(event).await {
                    Ok(_) => {}
                    Err(err) => listener.on_error(err).await,
                };
            }

            previous_number_of_events = number_of_events;
            tokio::time::sleep(period).await;
        }
    });
}
