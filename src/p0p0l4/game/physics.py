from ..numeric import F32, clamp, cos, sin
from .state import Ball


def update_ball(state, config, rng):
    if state.ball.timer > 0.0:
        state.ball.timer -= config.dt
        return
    ball = state.ball
    half = config.ball.half_size
    width, height = config.player.width, config.player.height
    ball.position.x += ball.dx * ball.velocity
    ball.position.y += ball.dy * ball.velocity
    if ball.position.y - half < 0.0:
        ball.position.y = half
        ball.dy = abs(ball.dy)
    elif ball.position.y + half > config.game_size:
        ball.position.y = config.game_size - half
        ball.dy = -abs(ball.dy)
    p1, p2 = state.player1, state.player2
    if (ball.dx < 0.0 and ball.position.x - half <= p1.position.x + width
            and ball.position.x + half >= p1.position.x
            and ball.position.y + half >= p1.position.y
            and ball.position.y - half <= p1.position.y + height):
        angle = Ball.calculate_hit_angle(ball.position.y, ball.dy, p1.position.y, height,
                                         config.ball.min_angle, config.ball.max_angle)
        ball.dx, ball.dy = cos(angle), sin(angle)
        ball.position.x = p1.position.x + width + half
        ball.velocity = clamp(ball.velocity + config.ball.velocity_add, F32(0.0), config.ball.velocity_max)
    if (ball.dx > 0.0 and ball.position.x + half >= p2.position.x
            and ball.position.x - half <= p2.position.x + width
            and ball.position.y + half >= p2.position.y
            and ball.position.y - half <= p2.position.y + height):
        angle = Ball.calculate_hit_angle(ball.position.y, ball.dy, p2.position.y, height,
                                         config.ball.min_angle, config.ball.max_angle)
        ball.dx, ball.dy = -cos(angle), sin(angle)
        ball.position.x = p2.position.x - half
        ball.velocity = clamp(ball.velocity + config.ball.velocity_add, F32(0.0), config.ball.velocity_max)
    handle_ball_goal(state, config, rng)


def handle_ball_goal(state, config, rng):
    if state.ball.position.x < 0.0:
        state.ball = Ball.new(config, rng)
        state.p2_goals += 1
    elif state.ball.position.x > config.game_size:
        state.ball = Ball.new(config, rng)
        state.p1_goals += 1
