use crate::models::position::Position;
use serde::Deserialize;
use crate::models::base_level::BaseLevel;
use crate::models::board_action::BoardAction;
use crate::models::game_config::GameConfig;
use crate::models::player_action::PlayerAction;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Base {
    pub position: Position,       // position of the base
    pub uid: u32,                 // uid of the base
    pub player: u32,              // owner of the base
    pub population: u32,          // current population of the base
    pub level: u32,               // level of the base
    pub units_until_upgrade: u32, // number of units required to upgrade
}

impl Default for Base {
    fn default() -> Self {
        Base {
            position: Position::default(),
            uid: 0,
            player: 0,
            population: 0,
            level: 0,
            units_until_upgrade: 0,
        }
    }
}

impl Base {
    pub fn population_in_n_ticks(&self, ticks: u32, config: &GameConfig, attacks: &Vec<BoardAction>) -> u32 {
        let mut population_in_future: i32 = self.population as i32;
        if self.uid != 0 { population_in_future + ticks as i32 * config.base_levels[self.level as usize].spawn_rate as i32; }
        for attack in attacks {
            if attack.arrival_in_ticks() > ticks {
                let val_on_target: i32 = attack.amount_at_target(&config.paths) as i32;
                if attack.player == self.player {
                    population_in_future += val_on_target;
                }
                else {
                    population_in_future -= val_on_target;
                }
            }
        }
        if population_in_future < 0 {return 0;}
        return population_in_future as u32;
    }

    pub fn distance_to(&self, base: &Base) -> u32 {
        return (((base.position.x - self.position.x).pow(2) - (base.position.y - self.position.y).pow(2) - (base.position.z - self.position.z).pow(2)) as f64).powf(1.0 / 3.0) as u32;
    }

    pub fn required_to_defeat(&self, base: &Base, attacks: &Vec<BoardAction>, game_config: &GameConfig) -> u32 {
        let d: u32 = self.distance_to(base);

        let pop = base.population_in_n_ticks(d, game_config, attacks);

        if d < game_config.paths.grace_period {return pop}

        return  pop + (d - game_config.paths.grace_period) * game_config.paths.death_rate
    }
}
