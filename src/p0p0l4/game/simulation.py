from ..numeric import F32, clamp
from . import bot, physics
from .state import Ball, Player, PlayerNum, Winner


def update(state, config, rng):
    if state.is_paused:
        return
    update_player1(state, config)
    bot.update(state, config, rng)
    physics.update_ball(state, config, rng)
    state.round_time -= config.dt
    if state.round_time <= 0.0:
        reset_round(state, config, rng)


def update_player1(state, config):
    state.player1.position.y += F32(state.player1.dx) * config.player.velocity
    state.player1.position.y = clamp(state.player1.position.y, F32(0.0), config.game_size - config.player.height)


def reset_players(state, config):
    state.player1 = Player.new(config, PlayerNum.P1)
    state.player2 = Player.new(config, PlayerNum.P2)


def reset_round(state, config, rng):
    state.round_time = config.round_time
    if state.p1_goals > state.p2_goals:
        state.p1_wins += 1
        state.round_winner = Winner.P1
    elif state.p1_goals < state.p2_goals:
        state.p2_wins += 1
        state.round_winner = Winner.P2
    else:
        state.round_winner = Winner.NO_WINNER
    state.p1_goals = state.p2_goals = 0
    state.round_count += 1
    state.ball = Ball.new(config, rng)
    reset_players(state, config)
