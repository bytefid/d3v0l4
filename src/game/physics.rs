use rand::{Rng, RngExt, rngs::StdRng};

use super::{
    config::GameConfig,
    state::{Ball, GameState, Point},
};

impl Ball {
    pub(super) fn new(config: &GameConfig, rng: &mut StdRng) -> Self {
        let (dx, dy) = Self::random_direction(rng);

        Self {
            dx,
            dy,
            velocity: config.ball.velocity,
            position: Point {
                x: config.game_size / 2.0,
                y: rng.random_range(config.ball.size..config.game_size - config.ball.size),
            },
            timer: config.ball_timer,
        }
    }

    fn random_direction(rng: &mut impl Rng) -> (f32, f32) {
        let angle = rng.random_range(-65.0_f32..65.0_f32).to_radians();
        let cos = angle.cos();
        let sin = angle.sin();
        let dx = if rng.random_bool(0.5) { -cos } else { cos };
        (dx, sin)
    }

    fn calculate_hit_angle(
        ball_y: f32,
        ball_dy: f32,
        paddle_y: f32,
        paddle_height: f32,
        min_angle: f32,
        max_angle: f32,
    ) -> f32 {
        let half_height = paddle_height / 2.0;
        let hit_pos = (ball_y - (paddle_y + half_height)) / half_height;
        let hit_pos = hit_pos.clamp(-1.0, 1.0);

        if hit_pos.abs() < 0.05 {
            if ball_dy >= 0.0 {
                min_angle
            } else {
                -min_angle
            }
        } else {
            hit_pos * max_angle
        }
    }
}

pub(super) fn update_ball(state: &mut GameState, config: &GameConfig, rng: &mut StdRng) {
    if state.ball.timer > 0.0 {
        state.ball.timer -= config.dt;
        return;
    }

    let ball_half = config.ball.half_size;
    let player_width = config.player.width;
    let player_height = config.player.height;
    let game_height = config.game_size;

    {
        let ball = &mut state.ball;

        ball.position.x += ball.dx * ball.velocity;
        ball.position.y += ball.dy * ball.velocity;

        if ball.position.y - ball_half < 0.0 {
            ball.position.y = ball_half;
            ball.dy = ball.dy.abs();
        } else if ball.position.y + ball_half > game_height {
            ball.position.y = game_height - ball_half;
            ball.dy = -ball.dy.abs();
        }

        let player1 = &state.player1;

        if ball.dx < 0.0
            && ball.position.x - ball_half <= player1.position.x + player_width
            && ball.position.x + ball_half >= player1.position.x
            && ball.position.y + ball_half >= player1.position.y
            && ball.position.y - ball_half <= player1.position.y + player_height
        {
            let angle = Ball::calculate_hit_angle(
                ball.position.y,
                ball.dy,
                player1.position.y,
                player_height,
                config.ball.min_angle,
                config.ball.max_angle,
            );

            ball.dx = angle.cos();
            ball.dy = angle.sin();
            ball.position.x = player1.position.x + player_width + ball_half;
            ball.velocity += config.ball.velocity_add;
            ball.velocity = ball.velocity.clamp(0.0, config.ball.velocity_max);
        }

        let player2 = &state.player2;

        if ball.dx > 0.0
            && ball.position.x + ball_half >= player2.position.x
            && ball.position.x - ball_half <= player2.position.x + player_width
            && ball.position.y + ball_half >= player2.position.y
            && ball.position.y - ball_half <= player2.position.y + player_height
        {
            let angle = Ball::calculate_hit_angle(
                ball.position.y,
                ball.dy,
                player2.position.y,
                player_height,
                config.ball.min_angle,
                config.ball.max_angle,
            );

            ball.dx = -angle.cos();
            ball.dy = angle.sin();
            ball.position.x = player2.position.x - ball_half;
            ball.velocity += config.ball.velocity_add;
            ball.velocity = ball.velocity.clamp(0.0, config.ball.velocity_max);
        }
    }

    handle_ball_goal(state, config, rng);
}

fn handle_ball_goal(state: &mut GameState, config: &GameConfig, rng: &mut StdRng) {
    if state.ball.position.x < 0.0 {
        state.ball = Ball::new(config, rng);
        state.p2_goals += 1;
    } else if state.ball.position.x > config.game_size {
        state.ball = Ball::new(config, rng);
        state.p1_goals += 1;
    }
}
