use rand::{RngExt, rngs::StdRng};

use super::{
    config::GameConfig,
    state::{Ball, GameState, Player},
};

pub(super) struct Bot {
    target_y: f32,
    timer: u32,
}

impl Bot {
    pub(super) fn new(config: &GameConfig) -> Self {
        Self {
            target_y: config.game_size / 2.0,
            timer: 0,
        }
    }

    fn predict_ball_y(config: &GameConfig, bot: &Player, ball: &Ball) -> f32 {
        let half = config.ball.half_size;

        if ball.dx.abs() < f32::EPSILON {
            return ball.position.y;
        }

        let contact_x = bot.position.x - half;
        let raw_y = ball.position.y + ball.dy / ball.dx * (contact_x - ball.position.x);

        let h = config.game_size - config.ball.size;
        let m = (raw_y - half).rem_euclid(2.0 * h);
        let y = if m > h { 2.0 * h - m } else { m };

        y + half
    }
}

pub(super) fn update(state: &mut GameState, config: &GameConfig, rng: &mut StdRng) {
    let center = state.player2.position.y + config.player.height / 2.0;

    if state.bot.timer == 0 {
        state.bot.timer = config.bot.reaction_ticks;

        state.bot.target_y = if state.ball.dx > 0.0 {
            if state.ball.timer <= 0.0 {
                Bot::predict_ball_y(config, &state.player2, &state.ball)
                    + rng.random_range(-config.bot.error_px..=config.bot.error_px)
            } else {
                config.game_size / 2.0
            }
        } else {
            config.game_size / 2.0
        };
    } else {
        state.bot.timer -= 1;
    }

    let diff = state.bot.target_y - center;
    let max_step = config.player.velocity * config.bot.speed_factor;

    let step = if diff.abs() > config.bot.deadzone {
        diff.clamp(-max_step, max_step)
    } else {
        0.0
    };

    let action = if step > 0.0 {
        1
    } else if step < 0.0 {
        -1
    } else {
        0
    };

    state.player_action(super::state::PlayerNum::P2, action);
    state.player2.position.y =
        (state.player2.position.y + step).clamp(0.0, config.game_size - config.player.height);
}
