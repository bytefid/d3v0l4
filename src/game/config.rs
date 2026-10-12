#[derive(Clone, Copy)]
pub(super) struct PlayerConfig {
    pub(super) velocity: f32,
    pub(super) width: f32,
    pub(super) height: f32,
}

#[derive(Clone, Copy)]
pub(super) struct BotConfig {
    pub(super) reaction_ticks: u32,
    pub(super) error_px: f32,
    pub(super) speed_factor: f32,
    pub(super) deadzone: f32,
}

#[derive(Clone, Copy)]
pub(super) struct BallConfig {
    pub(super) velocity: f32,
    pub(super) velocity_add: f32,
    pub(super) velocity_max: f32,
    pub(super) size: f32,
    pub(super) half_size: f32,
    pub(super) min_angle: f32,
    pub(super) max_angle: f32,
}

#[derive(Clone, Copy)]
pub struct GameConfig {
    pub(super) dt: f64,
    pub(super) sim_speed: f64,
    pub(super) game_size: f32,
    pub(super) round_time: f64,
    pub(super) ball_timer: f64,
    pub(super) seed: u64,
    pub(super) rast_size: usize,
    pub(super) player: PlayerConfig,
    pub(super) bot: BotConfig,
    pub(super) ball: BallConfig,
}

impl GameConfig {
    pub fn new() -> Self {
        let player = PlayerConfig {
            velocity: 14.0,
            width: 10.0,
            height: 60.0,
        };

        let ball_size = 10.0;
        let ball = BallConfig {
            velocity: 8.0,
            velocity_add: 1.5,
            velocity_max: 20.0,
            size: ball_size,
            half_size: ball_size / 2.0,
            min_angle: 5.0_f32.to_radians(),
            max_angle: 65.0_f32.to_radians(),
        };

        Self {
            dt: 1.0 / 50.0,
            sim_speed: 1.0,
            game_size: 1000.0,
            round_time: 60.0,
            ball_timer: 2.0,
            seed: 0x0,
            rast_size: 32,
            player,
            bot: BotConfig {
                reaction_ticks: 32,
                error_px: 50.0,
                speed_factor: 1.0,
                deadzone: 8.0,
            },
            ball,
        }
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    pub fn get_dt(&self) -> f64 {
        self.dt
    }

    pub fn get_rast_size(&self) -> usize {
        self.rast_size
    }
}
