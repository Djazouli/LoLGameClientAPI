//!
use crate::model::{
    Ace, BaronKill, ChampionKill, DragonKill, Event, FirstBlood, FirstBrick, GameEnd, GameStart,
    HeraldKill, InhibKilled, InhibRespawned, InhibRespawningSoon, MinionsSpawning, Multikill, Team,
    TurretKilled,
};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait EventListener: Send + Sync {
    type Error: Error + Send + Sync;
    async fn on_event(&mut self, event: Event) -> Result<(), Self::Error> {
        log::trace!("On event: {:?}", event);
        match event {
            Event::GameStart(event) => self.on_game_start(event).await,
            Event::GameEnd(event) => self.on_game_end(event).await,
            Event::MinionsSpawning(event) => self.on_minion_spawn(event).await,
            Event::FirstBrick(event) => self.on_first_brick(event).await,
            Event::FirstBlood(event) => self.on_first_blood(event).await,
            Event::TurretKilled(event) => self.on_turret_killed(event).await,
            Event::InhibKilled(event) => self.on_inhib_killed(event).await,
            Event::InhibRespawningSoon(event) => self.on_inhib_respawning_soon(event).await,
            Event::InhibRespawned(event) => self.on_inhib_respawned(event).await,
            Event::DragonKill(event) => self.on_dragon_killed(event).await,
            Event::HeraldKill(event) => self.on_herald_killed(event).await,
            Event::BaronKill(event) => self.on_baron_killed(event).await,
            Event::ChampionKill(event) => self.on_champion_kill(event).await,
            Event::Multikill(event) => self.on_multi_kill(event).await,
            Event::Ace(event) => self.on_ace(event).await,
        }
    }

    async fn on_game_start(&mut self, event: GameStart) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_game_end(&mut self, event: GameEnd) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_minion_spawn(&mut self, event: MinionsSpawning) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_first_brick(&mut self, event: FirstBrick) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_first_blood(&mut self, event: FirstBlood) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_turret_killed(&mut self, event: TurretKilled) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_inhib_killed(&mut self, event: InhibKilled) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn on_inhib_respawning_soon(
        &mut self,
        event: InhibRespawningSoon,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn on_inhib_respawned(&mut self, event: InhibRespawned) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_dragon_killed(&mut self, event: DragonKill) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_herald_killed(&mut self, event: HeraldKill) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_baron_killed(&mut self, event: BaronKill) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_champion_kill(&mut self, event: ChampionKill) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_multi_kill(&mut self, event: Multikill) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_ace(&mut self, event: Ace) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_error(&mut self, error: Self::Error) {
        log::error!("{}", error)
    }
}
