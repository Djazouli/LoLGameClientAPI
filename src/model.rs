//! Module containing all the structures that can be deserialized from the `https://127.0.0.1:2999/liveclientdata/` endpoint.
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    pub active_player: ActivePlayer,
    pub all_players: Vec<Player>,
    pub events: Events,
    pub game_data: GameData,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    pub abilities: Abilities,
    pub champion_stats: ChampionStats,
    pub current_gold: f64,
    pub full_runes: FullRunes,
    pub level: usize,
    pub summoner_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub champion_name: String,
    pub is_bot: bool,
    pub is_dead: bool,
    pub items: Vec<Item>,
    pub level: usize,
    pub position: String, // Enum ?
    pub raw_champion_name: String,
    pub respawn_timer: f64,
    pub runes: PartialRunes,
    pub scores: Scores,
    #[serde(rename = "skinID")]
    pub skin_id: usize,
    pub summoner_name: String,
    pub summoner_spells: SummonerSpells,
    pub team: Team,
}

// TODO
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub ward_score: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    pub summoner_spell_one: SummonerSpell,
    pub summoner_spell_two: SummonerSpell,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Team {
    #[serde(rename = "ORDER")]
    Order,
    #[serde(rename = "CHAOS")]
    Chaos,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    #[serde(rename = "E")]
    pub e: Ability,
    #[serde(rename = "Passive")]
    pub passive: Ability,
    #[serde(rename = "Q")]
    pub q: Ability,
    #[serde(rename = "R")]
    pub r: Ability,
    #[serde(rename = "W")]
    pub w: Ability,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability_level: Option<u8>, // May not have a level (on passive for example)
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_power: f64, // May not have a level (on passive for example)
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub bonus_armor_penetration_percent: f64,
    pub bonus_magic_penetration_percent: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub heal_shield_power: Option<f64>, // Optional because not in docs, but appears to be here anyway
    pub health_regen_rate: f64,
    pub life_steal: f64,
    pub magic_lethality: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub move_speed: f64,
    pub omnivamp: Option<f64>, // Optional because not in docs, but appears to be here anyway
    pub physical_lethality: f64,
    pub physical_vamp: Option<f64>, // Optional because not in docs, but appears to be here anyway
    pub resource_max: f64,
    pub resource_regen_rate: f64,
    pub resource_type: String, // Could be an enum I guess
    pub resource_value: f64,
    pub spell_vamp: f64,
    pub tenacity: f64,
}

/// Runes for the active player
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FullRunes {
    pub general_runes: Vec<Rune>,
    pub keystone: Rune,
    pub primary_rune_tree: RuneTree,
    pub secondary_rune_tree: RuneTree,
    pub stat_runes: [StatRunes; 3],
}

/// Runes for all the other players
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PartialRunes {
    pub keystone: Rune,
    pub primary_rune_tree: RuneTree,
    pub secondary_rune_tree: RuneTree,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub display_name: String,
    pub id: u16,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RuneTree {
    pub display_name: String,
    pub id: u16,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatRunes {
    pub id: u16,
    pub raw_description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub game_mode: GameMode,
    pub game_time: f64,
    pub map_name: String,
    pub map_number: usize,
    pub map_terrain: String, // enum ?
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GameMode {
    #[serde(rename = "CLASSIC")]
    Classic,
    #[serde(rename = "ARAM")]
    Aram,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "EventName")]
pub enum Event {
    GameStart(GameStart),
    GameEnd(GameEnd),
    MinionsSpawning(MinionsSpawning),
    FirstBrick(FirstBrick),
    FirstBlood(FirstBlood),
    TurretKilled(TurretKilled),
    InhibKilled(InhibKilled),
    InhibRespawningSoon(InhibRespawningSoon),
    InhibRespawned(InhibRespawned),
    DragonKill(DragonKill),
    HeraldKill(HeraldKill),
    BaronKill(BaronKill),
    ChampionKill(ChampionKill),
    Multikill(Multikill),
    Ace(Ace),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GameStart {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64, // TODO: time in seconds, but since when ? (GameStart is not at 0,00000)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GameEnd {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MinionsSpawning {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TurretKilled {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "TurretKilled")]
    pub turret_killed: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InhibKilled {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "InhibKilled")]
    pub inhib_killed: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InhibRespawningSoon {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "InhibRespawningSoon")]
    pub inhib_respawning_soon: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InhibRespawned {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "InhibRespawned")]
    pub inhib_respawned: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DragonKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
    #[serde(rename = "DragonType")]
    pub dragon_type: DragonType,
    #[serde(rename = "Stolen")]
    pub stolen: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FirstBrick {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FirstBlood {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "Recipient")]
    pub recipient: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeraldKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
    #[serde(rename = "Stolen")]
    pub stolen: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Ace {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "Acer")]
    pub acer: String,
    #[serde(rename = "AcingTeam")]
    pub acing_team: Team,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Multikill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "KillStreak")]
    pub kill_streak: u8,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ChampionKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "VictimName")]
    pub victim_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BaronKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
    #[serde(rename = "Stolen")]
    pub stolen: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum DragonType {
    Elder,
    Earth,
    Cloud,
    Infernal,
    Ocean,
    Hextech,
    Chemtech,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Events {
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
}
