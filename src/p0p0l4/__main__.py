import os
import time

os.environ.setdefault("PYGAME_HIDE_SUPPORT_PROMPT", "1")

import pygame

from .game import GameConfig, GameState, new_rng, rasterize, update
from .game.input import handle_input
from .game.render import GameRenderer
from .popola import PopolaConfig, raster_to_sense
from .numeric import F32


class Simulation:
    """The original fixed-timestep loop, including per-tick sensor updates."""

    def __init__(self, game_config=None, popola_config=None):
        self.game_config = game_config or GameConfig.new()
        self.popola_config = popola_config or PopolaConfig.new()
        self.rng = new_rng(self.game_config.get_seed())
        self.state = GameState.new(self.game_config, self.rng)
        self.accumulator = 0.0
        self.raster_buffer_curr = [0.0] * self.game_config.get_raster_size() ** 2
        self.raster_buffer_prev = self.raster_buffer_curr.copy()
        blocks = self.game_config.get_raster_size() // self.popola_config.get_sense_block_size()
        self.sense_buffer_on = [0.0] * blocks ** 2
        self.sense_buffer_off = self.sense_buffer_on.copy()
        rasterize(self.state, self.game_config, self.raster_buffer_curr, self.state.get_ball_timer() <= 0.0)

    def tick(self):
        update(self.state, self.game_config, self.rng)
        self.raster_buffer_curr, self.raster_buffer_prev = self.raster_buffer_prev, self.raster_buffer_curr
        rasterize(self.state, self.game_config, self.raster_buffer_curr, self.state.get_ball_timer() <= 0.0)
        raster_to_sense(self.popola_config, self.game_config, self.raster_buffer_curr, self.raster_buffer_prev,
                        self.sense_buffer_on, self.sense_buffer_off)

    def advance(self, frame_time):
        self.accumulator += frame_time
        sim_step = self.game_config.get_dt() / self.state.get_sim_speed()
        while self.accumulator >= sim_step:
            self.tick()
            self.accumulator -= sim_step


def main():
    pygame.display.init()
    pygame.freetype.init()
    try:
        pygame.display.set_caption("popola")
        screen = pygame.display.set_mode((1920, 1080), pygame.RESIZABLE)
        simulation = Simulation()
        renderer = GameRenderer.new(simulation.game_config, simulation.popola_config)
        clock = pygame.time.Clock()
        previous_time = time.perf_counter()
        down = set()
        keys = {pygame.K_SPACE: "space", pygame.K_UP: "up", pygame.K_DOWN: "down", pygame.K_LEFT: "left",
                pygame.K_RIGHT: "right", pygame.K_RSHIFT: "right_shift", pygame.K_w: "w", pygame.K_s: "s"}
        while simulation.state.is_running():
            pressed = set()
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    simulation.state.running = False
                elif event.type == pygame.KEYDOWN and event.key in keys:
                    key = keys[event.key]
                    if key not in down:
                        pressed.add(key)
                    down.add(key)
                elif event.type == pygame.KEYUP and event.key in keys:
                    down.discard(keys[event.key])
                elif event.type == pygame.WINDOWFOCUSLOST:
                    down.clear()
            if not simulation.state.is_running():
                break
            handle_input(simulation.state, pressed, down)
            now = time.perf_counter()
            # macroquad returns a f32 frame time before promoting it to f64.
            simulation.advance(float(F32(now - previous_time)))
            previous_time = now
            renderer.draw(screen, simulation.state, simulation.game_config, simulation.popola_config,
                          simulation.sense_buffer_on, simulation.sense_buffer_off, round(clock.get_fps()))
            pygame.display.flip()
            clock.tick(60)
    finally:
        pygame.quit()


if __name__ == "__main__":
    main()
