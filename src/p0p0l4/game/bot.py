from dataclasses import dataclass

from ..numeric import F32, clamp
from .state import PlayerNum


@dataclass(slots=True)
class Bot:
    target_y: F32
    timer: int = 0

    @classmethod
    def new(cls, config):
        return cls(config.game_size / 2.0)

    @staticmethod
    def predict_ball_y(config, bot, ball):
        half = config.ball.half_size
        if abs(ball.dx) < 2.0 ** -23:
            return ball.position.y
        contact_x = bot.position.x - half
        raw_y = ball.position.y + ball.dy / ball.dx * (contact_x - ball.position.x)
        height = config.game_size - config.ball.size
        remainder = (raw_y - half).rem_euclid(2.0 * height)
        y = 2.0 * height - remainder if remainder > height else remainder
        return y + half


def update(state, config, rng):
    center = state.player2.position.y + config.player.height / 2.0
    if state.bot.timer == 0:
        state.bot.timer = config.bot.reaction_ticks
        if state.ball.dx > 0.0 and state.ball.timer <= 0.0:
            state.bot.target_y = Bot.predict_ball_y(config, state.player2, state.ball) + rng.random_range(-config.bot.error_px, config.bot.error_px)
        else:
            state.bot.target_y = config.game_size / 2.0
    else:
        state.bot.timer -= 1
    diff = state.bot.target_y - center
    max_step = config.player.velocity * config.bot.speed_factor
    step = clamp(diff, -max_step, max_step) if abs(diff) > config.bot.deadzone else F32(0.0)
    state.player_action(PlayerNum.P2, 1 if step > 0.0 else -1 if step < 0.0 else 0)
    state.player2.position.y = clamp(state.player2.position.y + step, F32(0.0), config.game_size - config.player.height)
