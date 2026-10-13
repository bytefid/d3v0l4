from pathlib import Path
import math

import pygame
import pygame.freetype

from ..numeric import clamp

BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
GRAY = (130, 130, 130)
BLUE = (0, 119, 242)
RED = (229, 40, 56)
DARKPURPLE = (112, 30, 124)


class GameRenderer:
    def __init__(self, game_config, popola_config):
        blocks = game_config.get_raster_size() // popola_config.get_sense_block_size()
        self.sense_image = pygame.Surface((blocks, blocks))
        self.sense_image.fill(BLACK)
        self.font = pygame.freetype.Font(str(Path(__file__).resolve().parent.parent / "assets" / "ProggyClean.ttf"))
        self.font.origin = True  # Match macroquad's text baseline coordinates.

    @classmethod
    def new(cls, game_config, popola_config):
        return cls(game_config, popola_config)

    @staticmethod
    def rectangle(screen, x, y, width, height, color):
        # Rasterize the floating-point quad at pixel centers, as the GPU does.
        left, top = math.ceil(x - 0.5), math.ceil(y - 0.5)
        right, bottom = math.ceil(x + width - 0.5), math.ceil(y + height - 0.5)
        pygame.draw.rect(screen, color, (left, top, right - left, bottom - top))

    def text(self, screen, text, x, y, size, color):
        self.font.render_to(screen, (round(x), round(y)), text, color, size=max(1, int(size)))

    def draw(self, screen, state, game_config, popola_config, sense_buffer_on, sense_buffer_off, fps):
        screen.fill(BLACK)
        width, height = screen.get_size()
        scale = min(width / game_config.game_size, height / game_config.game_size)
        offset_x = (width - game_config.game_size * scale) / 2.0
        offset_y = (height - game_config.game_size * scale) / 2.0
        self.draw_net(screen, game_config, scale, offset_x, offset_y)
        for player, color in ((state.player1, BLUE), (state.player2, RED)):
            self.rectangle(screen, player.position.x * scale + offset_x, player.position.y * scale + offset_y,
                           game_config.player.width * scale, game_config.player.height * scale, color)
        if state.ball.timer <= 0.0:
            self.rectangle(screen, (state.ball.position.x - game_config.ball.half_size) * scale + offset_x,
                           (state.ball.position.y - game_config.ball.half_size) * scale + offset_y,
                           game_config.ball.size * scale, game_config.ball.size * scale, WHITE)
        self.draw_sense(screen, game_config, popola_config, sense_buffer_on, sense_buffer_off)
        # The original custom Camera2D has negative Y zoom: game geometry and
        # sensor texture are vertically inverted before returning to the default
        # camera for the pause overlay and HUD.
        screen.blit(pygame.transform.flip(screen, False, True), (0, 0))
        if state.is_paused:
            center = game_config.game_size / 2.0 * scale
            self.rectangle(screen, center - 255.0 * scale + offset_x, center - 75.0 * scale + offset_y,
                           550.0 * scale, 150.0 * scale, BLACK)
            self.text(screen, "PAUSED", center - 250.0 * scale + offset_x, center + 50.0 * scale + offset_y,
                      200.0 * scale, DARKPURPLE)
        self.text(screen, f"FPS: {fps}", 5, 15, 20, GRAY)
        self.draw_hud(screen, state)

    def draw_net(self, screen, config, scale, offset_x, offset_y):
        line_space = 20.0
        line_height = config.game_size / 30.0 - line_space
        line_width = 5.0
        x = (config.game_size / 2.0 - line_width / 2.0) * scale + offset_x
        step = (line_height + line_space) * scale
        y = line_height * scale + offset_y
        for _ in range(30):
            self.rectangle(screen, x, y, line_width * scale, line_height * scale, GRAY)
            y += step

    def draw_sense(self, screen, game_config, popola_config, sense_buffer_on, sense_buffer_off):
        blocks = game_config.get_raster_size() // popola_config.get_sense_block_size()
        assert len(sense_buffer_on) >= blocks ** 2
        assert len(sense_buffer_off) >= blocks ** 2
        for y in range(blocks):
            for x in range(blocks):
                index = y * blocks + x
                on = int(clamp(sense_buffer_on[index] * 2.0 * 255.0, 0.0, 255.0))
                off = int(clamp(sense_buffer_off[index] * 2.0 * 255.0, 0.0, 255.0))
                self.sense_image.set_at((x, y), (off, on, 0))
        display_size = int(game_config.raster_size * 6.0)
        screen.blit(pygame.transform.scale(self.sense_image, (display_size, display_size)), (10, 10))
        # macroquad's thickness=2 outline occupies one pixel inside the bounds.
        pygame.draw.rect(screen, WHITE, (9, 10, display_size + 2, display_size + 2), width=1)

    def draw_hud(self, screen, state):
        rows = (f"Round Count: {state.round_count}", f"Round Time: {state.round_time:.1f}",
                f"P1 Wins: {state.p1_wins}", f"P2 Wins: {state.p2_wins}",
                f"P1 Goals: {state.p1_goals}", f"P2 Goals: {state.p2_goals}")
        for text, y in zip(rows, range(40, 141, 20)):
            self.text(screen, text, 5, y, 30, GRAY)
