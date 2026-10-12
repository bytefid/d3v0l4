use rand::rngs::StdRng;

use super::{
    bot,
    config::GameConfig,
    physics,
    state::{Ball, GameState, Player, PlayerNum, Winner},
};

pub fn update(state: &mut GameState, config: &GameConfig, rng: &mut StdRng) {
    if state.is_paused {
        return;
    }

    update_player1(state, config);
    bot::update(state, config, rng);
    physics::update_ball(state, config, rng);

    state.round_time -= config.dt;
    if state.round_time <= 0.0 {
        reset_round(state, config, rng);
    }
}

fn update_player1(state: &mut GameState, config: &GameConfig) {
    state.player1.position.y += state.player1.dx as f32 * config.player.velocity;
    state.player1.position.y = state
        .player1
        .position
        .y
        .clamp(0.0, config.game_size - config.player.height);
}

fn reset_players(state: &mut GameState, config: &GameConfig) {
    state.player1 = Player::new(config, PlayerNum::P1);
    state.player2 = Player::new(config, PlayerNum::P2);
}

fn reset_round(state: &mut GameState, config: &GameConfig, rng: &mut StdRng) {
    state.round_time = config.round_time;

    state.round_winner = if state.p1_goals > state.p2_goals {
        state.p1_wins += 1;
        Winner::P1
    } else if state.p1_goals < state.p2_goals {
        state.p2_wins += 1;
        Winner::P2
    } else {
        Winner::NoWinner
    };

    state.p1_goals = 0;
    state.p2_goals = 0;
    state.round_count += 1;
    state.ball = Ball::new(config, rng);
    reset_players(state, config);
}
