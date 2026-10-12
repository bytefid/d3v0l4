use std::fmt::Write as _;

use macroquad::prelude::*;

use crate::popola::PopolaConfig;

use super::{config::GameConfig, state::GameState};

pub struct GameRenderer {
    sense_image: Image,
    sense_texture: Texture2D,
    text_buffer: String,
}

impl GameRenderer {
    pub fn new(game_config: &GameConfig, popola_config: &PopolaConfig) -> Self {
        let sense_image = Image::gen_image_color(
            (game_config.get_raster_size() / popola_config.get_sense_block_size()) as u16,
            (game_config.get_raster_size() / popola_config.get_sense_block_size()) as u16,
            BLACK,
        );
        let sense_texture = Texture2D::from_image(&sense_image);
        sense_texture.set_filter(FilterMode::Nearest);

        Self {
            sense_image,
            sense_texture,
            text_buffer: String::with_capacity(64),
        }
    }

    pub fn draw(
        &mut self,
        state: &GameState,
        game_config: &GameConfig,
        popola_config: &PopolaConfig,
        sense_buffer_on: &[f32],
        sense_buffer_off: &[f32],
    ) {
        Self::setup_draw();

        let game_scale = f32::min(
            screen_width() / game_config.game_size,
            screen_height() / game_config.game_size,
        );
        let game_offset_x = (screen_width() - game_config.game_size * game_scale) / 2.0;
        let game_offset_y = (screen_height() - game_config.game_size * game_scale) / 2.0;

        Self::draw_net(game_config, game_scale, game_offset_x, game_offset_y);
        Self::draw_p1(game_config, state, game_scale, game_offset_x, game_offset_y);
        Self::draw_p2(game_config, state, game_scale, game_offset_x, game_offset_y);
        Self::draw_ball(game_config, state, game_scale, game_offset_x, game_offset_y);
        self.draw_sense(
            game_config,
            popola_config,
            sense_buffer_on,
            sense_buffer_off,
        );

        set_default_camera();
        Self::draw_paused(game_config, state, game_scale, game_offset_x, game_offset_y);
        self.draw_fps();
        self.draw_hud(state);
    }

    fn setup_draw() {
        clear_background(BLACK);

        set_camera(&Camera2D {
            zoom: vec2(2.0 / screen_width(), -2.0 / screen_height()),
            target: vec2(screen_width() / 2.0, screen_height() / 2.0),
            ..Default::default()
        });
    }

    fn draw_sense(
        &mut self,
        game_config: &GameConfig,
        popola_config: &PopolaConfig,
        sense_buffer_on: &[f32],
        sense_buffer_off: &[f32],
    ) {
        let rast_size = game_config.get_raster_size();
        let block_size = popola_config.get_sense_block_size();
        let blocks_per_row = rast_size / block_size;
        let total_blocks = blocks_per_row * blocks_per_row;

        assert!(sense_buffer_on.len() >= total_blocks);
        assert!(sense_buffer_off.len() >= total_blocks);

        let gain = 2.0_f32;

        for y in 0..blocks_per_row {
            for x in 0..blocks_per_row {
                let index = y * blocks_per_row + x;

                let on_val = (sense_buffer_on[index] * gain * 255.0).clamp(0.0, 255.0) as u8;
                let off_val = (sense_buffer_off[index] * gain * 255.0).clamp(0.0, 255.0) as u8;

                let color = Color::from_rgba(off_val, on_val, 0, 255);

                self.sense_image.set_pixel(x as u32, y as u32, color);
            }
        }

        self.sense_texture.update(&self.sense_image);

        let display_size = game_config.raster_size as f32 * 6.0;

        draw_texture_ex(
            &self.sense_texture,
            10.0,
            10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(display_size, display_size)),
                ..Default::default()
            },
        );

        draw_rectangle_lines(
            9.0,
            10.0,
            display_size + 2.0,
            display_size + 2.0,
            2.0,
            WHITE,
        );
    }

    fn draw_net(config: &GameConfig, game_scale: f32, offset_x: f32, offset_y: f32) {
        let line_space = 20.0;
        let line_height = config.game_size / 30.0 - line_space;
        let line_width = 5.0;
        let x = (config.game_size / 2.0 - line_width / 2.0) * game_scale + offset_x;
        let step = (line_height + line_space) * game_scale;
        let mut y = line_height * game_scale + offset_y;

        for _ in 0..30 {
            draw_rectangle(
                x,
                y,
                line_width * game_scale,
                line_height * game_scale,
                GRAY,
            );
            y += step;
        }
    }

    fn draw_p1(
        config: &GameConfig,
        state: &GameState,
        game_scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        draw_rectangle(
            state.player1.position.x * game_scale + offset_x,
            state.player1.position.y * game_scale + offset_y,
            config.player.width * game_scale,
            config.player.height * game_scale,
            BLUE,
        );
    }

    fn draw_p2(
        config: &GameConfig,
        state: &GameState,
        game_scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        draw_rectangle(
            state.player2.position.x * game_scale + offset_x,
            state.player2.position.y * game_scale + offset_y,
            config.player.width * game_scale,
            config.player.height * game_scale,
            RED,
        );
    }

    fn draw_ball(
        config: &GameConfig,
        state: &GameState,
        game_scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        if state.ball.timer <= 0.0 {
            draw_rectangle(
                (state.ball.position.x - config.ball.half_size) * game_scale + offset_x,
                (state.ball.position.y - config.ball.half_size) * game_scale + offset_y,
                config.ball.size * game_scale,
                config.ball.size * game_scale,
                WHITE,
            );
        }
    }

    fn draw_paused(
        config: &GameConfig,
        state: &GameState,
        game_scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        if !state.is_paused {
            return;
        }

        draw_rectangle(
            config.game_size / 2.0 * game_scale - 255.0 * game_scale + offset_x,
            config.game_size / 2.0 * game_scale - 75.0 * game_scale + offset_y,
            550.0 * game_scale,
            150.0 * game_scale,
            BLACK,
        );

        draw_text(
            "PAUSED",
            config.game_size / 2.0 * game_scale - 250.0 * game_scale + offset_x,
            config.game_size / 2.0 * game_scale + 50.0 * game_scale + offset_y,
            200.0 * game_scale,
            DARKPURPLE,
        );
    }

    fn draw_fps(&mut self) {
        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "FPS: {}", get_fps());
        draw_text(&self.text_buffer, 5.0, 15.0, 20.0, GRAY);
    }

    fn draw_hud(&mut self, state: &GameState) {
        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "Round Count: {}", state.round_count);
        draw_text(&self.text_buffer, 5.0, 40.0, 30.0, GRAY);

        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "Round Time: {:.1}", state.round_time);
        draw_text(&self.text_buffer, 5.0, 60.0, 30.0, GRAY);

        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "P1 Wins: {}", state.p1_wins);
        draw_text(&self.text_buffer, 5.0, 80.0, 30.0, GRAY);

        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "P2 Wins: {}", state.p2_wins);
        draw_text(&self.text_buffer, 5.0, 100.0, 30.0, GRAY);

        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "P1 Goals: {}", state.p1_goals);
        draw_text(&self.text_buffer, 5.0, 120.0, 30.0, GRAY);

        self.text_buffer.clear();
        let _ = write!(&mut self.text_buffer, "P2 Goals: {}", state.p2_goals);
        draw_text(&self.text_buffer, 5.0, 140.0, 30.0, GRAY);
    }
}
