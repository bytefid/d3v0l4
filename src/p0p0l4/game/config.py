from dataclasses import dataclass, field

from ..numeric import F32


@dataclass(slots=True)
class PlayerConfig:
    velocity: F32 = F32(14.0)
    width: F32 = F32(10.0)
    height: F32 = F32(60.0)


@dataclass(slots=True)
class BotConfig:
    reaction_ticks: int = 32
    error_px: F32 = F32(50.0)
    speed_factor: F32 = F32(1.0)
    deadzone: F32 = F32(8.0)


@dataclass(slots=True)
class BallConfig:
    velocity: F32 = F32(8.0)
    velocity_add: F32 = F32(1.5)
    velocity_max: F32 = F32(20.0)
    size: F32 = F32(10.0)
    half_size: F32 = F32(5.0)
    min_angle: F32 = F32(5.0).to_radians()
    max_angle: F32 = F32(65.0).to_radians()


@dataclass(slots=True)
class GameConfig:
    dt: float = 1.0 / 50.0
    sim_speed: float = 1.0
    game_size: F32 = F32(1000.0)
    round_time: float = 60.0
    ball_timer: float = 2.0
    seed: int = 0
    raster_size: int = 32
    player: PlayerConfig = field(default_factory=PlayerConfig)
    bot: BotConfig = field(default_factory=BotConfig)
    ball: BallConfig = field(default_factory=BallConfig)

    @classmethod
    def new(cls):
        return cls()

    def get_seed(self):
        return self.seed

    def get_dt(self):
        return self.dt

    def get_raster_size(self):
        return self.raster_size
