use macroquad::prelude::{KeyCode, is_key_down, is_key_pressed};

use super::state::{GameState, PlayerNum};

pub fn handle_input(state: &mut GameState) {
    if is_key_pressed(KeyCode::Space) {
        state.is_paused = !state.is_paused;
    }

    if is_key_pressed(KeyCode::Up) {
        state.sim_speed += 5.0;
        if state.sim_speed > 275_000.0 {
            state.sim_speed = 275_000.0;
        }
    } else if is_key_pressed(KeyCode::Down) {
        state.sim_speed -= 5.0;
        if state.sim_speed < 1.0 {
            state.sim_speed = 1.0;
        }
    }

    if is_key_pressed(KeyCode::Up) && is_key_down(KeyCode::RightShift) {
        state.sim_speed += 1000.0;
        if state.sim_speed > 275_000.0 {
            state.sim_speed = 275_000.0;
        }
    } else if is_key_pressed(KeyCode::Down) && is_key_down(KeyCode::RightShift) {
        state.sim_speed -= 1000.0;
        if state.sim_speed < 1.0 {
            state.sim_speed = 1.0;
        }
    }

    if is_key_pressed(KeyCode::Left) && is_key_down(KeyCode::RightShift) {
        state.sim_speed = 1.0;
    } else if is_key_pressed(KeyCode::Right) && is_key_down(KeyCode::RightShift) {
        state.sim_speed = 275_000.0;
    }

    let action = if is_key_down(KeyCode::W) {
        1
    } else if is_key_down(KeyCode::S) {
        -1
    } else {
        0
    };

    state.player_action(PlayerNum::P1, action);
}
