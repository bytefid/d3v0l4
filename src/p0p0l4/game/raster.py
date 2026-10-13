import math

from ..numeric import F32, clamp


def rasterize_rect(config, raster_buffer, x, y, width, height, value):
    scale = F32(config.raster_size) / config.game_size
    limit = config.raster_size
    x0 = int(clamp(math.floor(x * scale), 0, limit))
    y0 = int(clamp(math.floor(y * scale), 0, limit))
    x1 = int(clamp(math.ceil((x + width) * scale), 0, limit))
    y1 = int(clamp(math.ceil((y + height) * scale), 0, limit))
    for gy in range(y0, y1):
        start = gy * limit
        for gx in range(x0, x1):
            raster_buffer[start + gx] = value


def rasterize(state, config, raster_buffer, rasterize_ball):
    total_pixels = config.raster_size ** 2
    assert len(raster_buffer) >= total_pixels
    raster_buffer[:total_pixels] = [0.0] * total_pixels
    for player in (state.player1, state.player2):
        rasterize_rect(config, raster_buffer, player.position.x, player.position.y,
                       config.player.width, config.player.height, 0.75)
    if rasterize_ball:
        rasterize_rect(config, raster_buffer, state.ball.position.x - config.ball.half_size,
                       state.ball.position.y - config.ball.half_size, config.ball.size, config.ball.size, 1.0)
