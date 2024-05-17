use crate::models::progress::Progress;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::game_config::GameConfig;
use crate::models::path_config::PathConfig;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct BoardAction {
    pub src: u32,           // uid of source base
    pub dest: u32,          // uid of destination base
    pub amount: u32,        // number of bits moved
    pub uuid: Uuid,         // uuid of the action
    pub player: u32,        // id of the player who took the action
    pub progress: Progress, // progress off the action
}

impl Default for BoardAction {
    fn default() -> Self {
        BoardAction {
            src: 0,
            dest: 0,
            amount: 0,
            uuid: Uuid::default(),
            player: 0,
            progress: Progress::default(),
        }
    }
}

impl BoardAction {
    pub fn arrival_in_ticks(&self) -> u32 {
        return self.progress.distance - self.progress.traveled;
    }
    pub fn amount_at_target(&self, config: &PathConfig) -> u32 {
        if self.progress.distance < config.grace_period { return self.amount; }
        let deaths: u32 = config.death_rate * (self.progress.distance - config.grace_period);
        if deaths > self.amount { return 0 }
        return self.amount - deaths;
    }
}
