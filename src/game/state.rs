use rand::rngs::StdRng;

use super::{bot::Bot, config::GameConfig};

#[derive(Clone, Copy)]
pub(super) struct Point {
    pub(super) x: f32,
    pub(super) y: f32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum Winner {
    NoWinner,
    P1,
    P2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerNum {
    P1,
    P2,
}

pub(super) struct Player {
    pub(super) dx: i8,
    pub(super) position: Point,
}

pub(super) struct Ball {
    pub(super) dx: f32,
    pub(super) dy: f32,
    pub(super) velocity: f32,
    pub(super) position: Point,
    pub(super) timer: f64,
}

impl Player {
    pub(super) fn new(config: &GameConfig, player_num: PlayerNum) -> Self {
        let position = match player_num {
            PlayerNum::P1 => Point {
                x: config.player.width,
                y: config.game_size / 2.0 - config.player.height / 2.0,
            },
            PlayerNum::P2 => Point {
                x: config.game_size - config.player.width * 2.0,
                y: config.game_size / 2.0 - config.player.height / 2.0,
            },
        };

        Self { dx: 0, position }
    }
}

pub struct GameState {
    pub(super) is_running: bool,
    pub(super) is_paused: bool,
    pub(super) sim_speed: f64,

    pub(super) round_winner: Winner,
    pub(super) round_count: usize,
    pub(super) round_time: f64,
    pub(super) p1_goals: usize,
    pub(super) p2_goals: usize,
    pub(super) p1_wins: usize,
    pub(super) p2_wins: usize,

    pub(super) player1: Player,
    pub(super) player2: Player,
    pub(super) bot: Bot,
    pub(super) ball: Ball,
}

impl GameState {
    pub fn new(config: &GameConfig, rng: &mut StdRng) -> Self {
        Self {
            is_running: true,
            is_paused: false,
            sim_speed: config.sim_speed,

            round_winner: Winner::NoWinner,
            round_count: 0,
            round_time: config.round_time,
            p1_goals: 0,
            p2_goals: 0,
            p1_wins: 0,
            p2_wins: 0,

            player1: Player::new(config, PlayerNum::P1),
            player2: Player::new(config, PlayerNum::P2),
            bot: Bot::new(config),
            ball: Ball::new(config, rng),
        }
    }

    pub fn get_ball_timer(&self) -> f64 {
        self.ball.timer
    }

    pub fn get_sim_speed(&self) -> f64 {
        self.sim_speed
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub(super) fn player_action(&mut self, player_num: PlayerNum, action: i8) {
        match player_num {
            PlayerNum::P1 => self.player1.dx = action,
            PlayerNum::P2 => self.player2.dx = action,
        }
    }
}
