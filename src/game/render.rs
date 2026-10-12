use std::fmt::Write as _;

use macroquad::prelude::*;

use super::{config::GameConfig, state::GameState};

pub struct GameRenderer {
    raster_image: Image,
    raster_texture: Texture2D,
    text_buffer: String,
}

impl GameRenderer {
    pub fn new(config: &GameConfig) -> Self {
        let raster_image =
            Image::gen_image_color(config.rast_size as u16, config.rast_size as u16, BLACK);
        let raster_texture = Texture2D::from_image(&raster_image);
        raster_texture.set_filter(FilterMode::Nearest);

        Self {
            raster_image,
            raster_texture,
            text_buffer: String::with_capacity(64),
        }
    }

    pub fn draw(&mut self, state: &GameState, config: &GameConfig, raster_buffer: &[f32]) {
        Self::setup_draw();

        let game_scale = f32::min(
            screen_width() / config.game_size,
            screen_height() / config.game_size,
        );
        let game_offset_x = (screen_width() - config.game_size * game_scale) / 2.0;
        let game_offset_y = (screen_height() - config.game_size * game_scale) / 2.0;

        Self::draw_net(config, game_scale, game_offset_x, game_offset_y);
        Self::draw_p1(config, state, game_scale, game_offset_x, game_offset_y);
        Self::draw_p2(config, state, game_scale, game_offset_x, game_offset_y);
        Self::draw_ball(config, state, game_scale, game_offset_x, game_offset_y);
        self.draw_rasterized(config, raster_buffer);

        set_default_camera();
        Self::draw_paused(config, state, game_scale, game_offset_x, game_offset_y);
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

    fn draw_rasterized(&mut self, config: &GameConfig, raster_buffer: &[f32]) {
        let total_pixels = config.rast_size * config.rast_size;
        assert!(raster_buffer.len() >= total_pixels);

        for y in 0..config.rast_size {
            for x in 0..config.rast_size {
                let index = y * config.rast_size + x;
                let brightness = (raster_buffer[index] * 255.0).clamp(0.0, 255.0) as u8;
                let color = Color::from_rgba(brightness, brightness, brightness, 255);
                self.raster_image.set_pixel(x as u32, y as u32, color);
            }
        }

        self.raster_texture.update(&self.raster_image);

        draw_texture_ex(
            &self.raster_texture,
            10.0,
            10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    config.rast_size as f32 * 8.0,
                    config.rast_size as f32 * 8.0,
                )),
                ..Default::default()
            },
        );

        draw_rectangle_lines(
            9.0,
            10.0,
            config.rast_size as f32 * 8.0 + 2.0,
            config.rast_size as f32 * 8.0 + 2.0,
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
