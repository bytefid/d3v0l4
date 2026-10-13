from dataclasses import dataclass
from enum import IntEnum

from ..numeric import F32, clamp, cos, sin


@dataclass(slots=True)
class Point:
    x: F32
    y: F32


class Winner(IntEnum):
    NO_WINNER = 0
    P1 = 1
    P2 = 2


class PlayerNum(IntEnum):
    P1 = 1
    P2 = 2


@dataclass(slots=True)
class Player:
    dx: int
    position: Point

    @classmethod
    def new(cls, config, player_num):
        x = config.player.width if player_num == PlayerNum.P1 else config.game_size - config.player.width * 2.0
        return cls(0, Point(x, config.game_size / 2.0 - config.player.height / 2.0))


@dataclass(slots=True)
class Ball:
    dx: F32
    dy: F32
    velocity: F32
    position: Point
    timer: float

    @classmethod
    def new(cls, config, rng):
        dx, dy = cls.random_direction(rng)
        return cls(dx, dy, config.ball.velocity,
                   Point(config.game_size / 2.0, rng.random_range(config.ball.size, config.game_size - config.ball.size)),
                   config.ball_timer)

    @staticmethod
    def random_direction(rng):
        angle = rng.random_range(-65.0, 65.0).to_radians()
        cosine, sine = cos(angle), sin(angle)
        return (-cosine if rng.random_bool(0.5) else cosine), sine

    @staticmethod
    def calculate_hit_angle(ball_y, ball_dy, paddle_y, paddle_height, min_angle, max_angle):
        half_height = paddle_height / 2.0
        hit_pos = clamp((ball_y - (paddle_y + half_height)) / half_height, F32(-1.0), F32(1.0))
        if abs(hit_pos) < F32(0.05):
            return min_angle if ball_dy >= 0.0 else -min_angle
        return hit_pos * max_angle


@dataclass(slots=True)
class GameState:
    running: bool
    is_paused: bool
    sim_speed: float
    round_winner: Winner
    round_count: int
    round_time: float
    p1_goals: int
    p2_goals: int
    p1_wins: int
    p2_wins: int
    player1: Player
    player2: Player
    bot: "Bot"
    ball: Ball

    @classmethod
    def new(cls, config, rng):
        from .bot import Bot
        return cls(True, False, config.sim_speed, Winner.NO_WINNER, 0, config.round_time, 0, 0, 0, 0,
                   Player.new(config, PlayerNum.P1), Player.new(config, PlayerNum.P2),
                   Bot.new(config), Ball.new(config, rng))

    def get_ball_timer(self):
        return self.ball.timer

    def get_sim_speed(self):
        return self.sim_speed

    def is_running(self):
        return self.running

    def player_action(self, player_num, action):
        player = self.player1 if player_num == PlayerNum.P1 else self.player2
        player.dx = action
