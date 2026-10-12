use crate::game::GameConfig;

use super::PopolaConfig;

pub fn raster_to_sense(
    popola_config: &PopolaConfig,
    game_config: &GameConfig,
    raster_buffer_curr: &[f32],
    raster_buffer_prev: &[f32],
    sense_buffer_on: &mut [f32],
    sense_buffer_off: &mut [f32],
) {
    let rast_size = game_config.get_raster_size();
    let block_size = popola_config.get_sense_block_size();
    let blocks_per_row = rast_size / block_size;
    let total_blocks = blocks_per_row * blocks_per_row;

    assert!(sense_buffer_on.len() >= total_blocks);
    assert!(sense_buffer_off.len() >= total_blocks);

    let pixels_in_block = (block_size * block_size) as f32;

    for by in 0..blocks_per_row {
        for bx in 0..blocks_per_row {
            let mut sum_on = 0.0_f32;
            let mut sum_off = 0.0_f32;

            for dy in 0..block_size {
                for dx in 0..block_size {
                    let px = bx * block_size + dx;
                    let py = by * block_size + dy;
                    let pixel_idx = py * rast_size + px;

                    let delta = raster_buffer_curr[pixel_idx] - raster_buffer_prev[pixel_idx];

                    sum_on += delta.max(0.0);
                    sum_off += (-delta).max(0.0);
                }
            }

            let block_idx = by * blocks_per_row + bx;

            sense_buffer_on[block_idx] = sum_on / pixels_in_block;
            sense_buffer_off[block_idx] = sum_off / pixels_in_block;
        }
    }
}
