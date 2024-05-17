use crate::models::{game_state::GameState, player_action::PlayerAction};
use crate::models::base::Base;

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    let mut attacks: Vec<PlayerAction> = Vec::new();

    let mut own_bases: Vec<Base> = Vec::new();
    let mut opponent_bases: Vec<Base> = Vec::new();

    for base in game_state.bases {
        if base.player == game_state.game.player {
            own_bases.push(base);
        }
        else {
            opponent_bases.push(base);
        }
    }

    for base in own_bases {
        let mut target: Option<(Base, u32)> = None;
        for opponent in opponent_bases.clone() {
            let req = base.required_to_defeat(&opponent, &game_state.actions, &game_state.config);
            if req < base.population {
                if let Some(target_some) = target {
                    if target_some.1 > req {
                        target = Some((opponent, req));
                    }
                }
                else {
                    target = Some((opponent, req));
                }
            }
        }

        if let Some(target) = target {
            if base.population_in_n_ticks(base.distance_to(&target.0), &game_state.config, &game_state.actions) > target.1 {
                attacks.push(PlayerAction {
                    src: base.uid,
                    dest: target.0.uid,
                    amount: target.1,
                });
            }
            else if base.population > game_state.config.base_levels[base.level as usize].max_population - 5 {
                attacks.push(PlayerAction {
                    src: base.uid,
                    dest: base.uid,
                    amount: game_state.config.base_levels[base.level as usize].spawn_rate,
                });
            }
        }
        else if base.population > game_state.config.base_levels[base.level as usize].max_population - 5 {
            attacks.push(PlayerAction {
                src: base.uid,
                dest: base.uid,
                amount: game_state.config.base_levels[base.level as usize].spawn_rate,
            });
        }
    }
    return attacks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }
}
