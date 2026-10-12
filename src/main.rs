mod game;
mod popola;

use game::{GameConfig, GameRenderer, GameState, new_rng};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let config = GameConfig::new();
    let mut rng = new_rng(config.get_seed());
    let mut state = GameState::new(&config, &mut rng);
    let mut renderer = GameRenderer::new(&config);

    let mut accumulator = 0.0;

    let mut sensor_buffer_curr = vec![0.0_f32; config.get_rast_size() * config.get_rast_size()];
    let mut sensor_buffer_prev = sensor_buffer_curr.clone();

    game::rasterize(
        &state,
        &config,
        &mut sensor_buffer_curr,
        state.get_ball_timer() <= 0.0,
    );

    loop {
        if !state.is_running() {
            break;
        }

        game::handle_input(&mut state);

        let frame_time = get_frame_time() as f64;
        accumulator += frame_time;

        let sim_step = config.get_dt() / state.get_sim_speed();

        while accumulator >= sim_step {
            game::update(&mut state, &config, &mut rng);

            std::mem::swap(&mut sensor_buffer_curr, &mut sensor_buffer_prev);

            game::rasterize(
                &state,
                &config,
                &mut sensor_buffer_curr,
                state.get_ball_timer() <= 0.0,
            );

            accumulator -= sim_step;
        }

        renderer.draw(&state, &config, &sensor_buffer_curr);
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "popola".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: true,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            apple_gfx_api: miniquad::conf::AppleGfxApi::Metal,
            ..Default::default()
        },
        ..Default::default()
    }
}
