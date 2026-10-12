mod game;
mod popola;

use game::{GameConfig, GameRenderer, GameState, new_rng};
use macroquad::prelude::*;
use popola::PopolaConfig;

#[macroquad::main(window_conf)]
async fn main() {
    let game_config = GameConfig::new();
    let popola_config = PopolaConfig::new();

    let mut rng = new_rng(game_config.get_seed());
    let mut state = GameState::new(&game_config, &mut rng);
    let mut renderer = GameRenderer::new(&game_config, &popola_config);

    let mut accumulator = 0.0;

    let mut raster_buffer_curr =
        vec![0.0_f32; game_config.get_raster_size() * game_config.get_raster_size()];
    let mut raster_buffer_prev = raster_buffer_curr.clone();

    let mut sense_buffer_on =
        vec![
            0.0_f32;
            (game_config.get_raster_size() / popola_config.get_sense_block_size())
                * (game_config.get_raster_size() / popola_config.get_sense_block_size())
        ];
    let mut sense_buffer_off = sense_buffer_on.clone();

    game::rasterize(
        &state,
        &game_config,
        &mut raster_buffer_curr,
        state.get_ball_timer() <= 0.0,
    );

    loop {
        if !state.is_running() {
            break;
        }

        game::handle_input(&mut state);

        let frame_time = get_frame_time() as f64;
        accumulator += frame_time;

        let sim_step = game_config.get_dt() / state.get_sim_speed();

        while accumulator >= sim_step {
            game::update(&mut state, &game_config, &mut rng);

            std::mem::swap(&mut raster_buffer_curr, &mut raster_buffer_prev);

            game::rasterize(
                &state,
                &game_config,
                &mut raster_buffer_curr,
                state.get_ball_timer() <= 0.0,
            );

            popola::raster_to_sense(
                &popola_config,
                &game_config,
                &raster_buffer_curr,
                &raster_buffer_prev,
                &mut sense_buffer_on,
                &mut sense_buffer_off,
            );

            accumulator -= sim_step;
        }

        renderer.draw(
            &state,
            &game_config,
            &popola_config,
            &sense_buffer_on,
            &sense_buffer_off,
        );
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
