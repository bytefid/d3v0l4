use super::{config::GameConfig, state::GameState};

fn rasterize_rect(
    config: &GameConfig,
    raster_buffer: &mut [f32],
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: f32,
) {
    let scale = config.rast_size as f32 / config.game_size;
    let limit = config.rast_size as f32;

    let x0 = (x * scale).floor().clamp(0.0, limit) as usize;
    let y0 = (y * scale).floor().clamp(0.0, limit) as usize;
    let x1 = ((x + width) * scale).ceil().clamp(0.0, limit) as usize;
    let y1 = ((y + height) * scale).ceil().clamp(0.0, limit) as usize;

    for gy in y0..y1 {
        for gx in x0..x1 {
            raster_buffer[gy * config.rast_size + gx] = value;
        }
    }
}

pub fn rasterize(
    state: &GameState,
    config: &GameConfig,
    raster_buffer: &mut [f32],
    rasterize_ball: bool,
) {
    let total_pixels = config.rast_size * config.rast_size;
    assert!(raster_buffer.len() >= total_pixels);
    raster_buffer[..total_pixels].fill(0.0);

    rasterize_rect(
        config,
        raster_buffer,
        state.player1.position.x,
        state.player1.position.y,
        config.player.width,
        config.player.height,
        1.0,
    );

    rasterize_rect(
        config,
        raster_buffer,
        state.player2.position.x,
        state.player2.position.y,
        config.player.width,
        config.player.height,
        1.0,
    );

    if rasterize_ball {
        rasterize_rect(
            config,
            raster_buffer,
            state.ball.position.x - config.ball.half_size,
            state.ball.position.y - config.ball.half_size,
            config.ball.size,
            config.ball.size,
            0.5,
        );
    }
}
