import os
import unittest
from unittest.mock import patch

os.environ.setdefault("SDL_VIDEODRIVER", "dummy")
os.environ.setdefault("SDL_AUDIODRIVER", "dummy")
os.environ.setdefault("PYGAME_HIDE_SUPPORT_PROMPT", "1")

import pygame
import pygame.freetype

from p0p0l4.__main__ import Simulation, main
from p0p0l4.game import GameConfig, GameState, new_rng, rasterize, update
from p0p0l4.game.bot import Bot
from p0p0l4.game.input import handle_input
from p0p0l4.game.physics import update_ball
from p0p0l4.game.render import BLACK, BLUE, RED, WHITE, GameRenderer
from p0p0l4.game.simulation import reset_round
from p0p0l4.game.state import Ball, Point, Winner
from p0p0l4.numeric import F32
from p0p0l4.popola import PopolaConfig, raster_to_sense


class GameTests(unittest.TestCase):
    def setUp(self):
        self.config = GameConfig.new()
        self.rng = new_rng(self.config.seed)
        self.state = GameState.new(self.config, self.rng)

    def test_pause_freezes_game_and_rng(self):
        self.state.is_paused = True
        before = repr(self.state)
        update(self.state, self.config, self.rng)
        self.assertEqual(repr(self.state), before)
        other = new_rng(self.config.seed)
        GameState.new(self.config, other)
        self.assertEqual(self.rng.next_u64(), other.next_u64())

    def test_serve_delay_does_not_move_ball(self):
        before = repr(self.state.ball.position)
        for _ in range(100):
            update_ball(self.state, self.config, self.rng)
        self.assertEqual(repr(self.state.ball.position), before)
        self.assertLessEqual(self.state.ball.timer, 0.0)
        update_ball(self.state, self.config, self.rng)
        self.assertNotEqual(repr(self.state.ball.position), before)

    def test_wall_reflections(self):
        for y, dy, expected_y, expected_dy in ((5, -1, 5, 1), (995, 1, 995, -1)):
            with self.subTest(y=y):
                self.state.ball = Ball(F32(0), F32(dy), F32(8), Point(F32(500), F32(y)), 0.0)
                update_ball(self.state, self.config, self.rng)
                self.assertEqual(self.state.ball.position.y, expected_y)
                self.assertEqual(self.state.ball.dy, expected_dy)

    def test_paddle_collisions_and_speed_limit(self):
        for x, dx, expected_x, sign in ((28, -1, 25, 1), (972, 1, 975, -1)):
            with self.subTest(x=x):
                self.state.ball = Ball(F32(dx), F32(0), F32(19), Point(F32(x), F32(500)), 0.0)
                update_ball(self.state, self.config, self.rng)
                self.assertEqual(self.state.ball.position.x, expected_x)
                self.assertGreater(self.state.ball.dx * sign, 0)
                self.assertEqual(self.state.ball.velocity, 20)
                self.assertGreater(abs(self.state.ball.dy), 0)

    def test_goals_respawn_with_delay(self):
        for x, dx, scorer in ((1, -1, "p2_goals"), (999, 1, "p1_goals")):
            with self.subTest(x=x):
                self.state.ball = Ball(F32(dx), F32(0), F32(8), Point(F32(x), F32(100)), 0.0)
                update_ball(self.state, self.config, self.rng)
                self.assertEqual(getattr(self.state, scorer), 1)
                self.assertEqual(self.state.ball.position.x, 500)
                self.assertEqual(self.state.ball.timer, 2.0)

    def test_round_wins_draws_and_reset(self):
        for goals, winner, wins in (((2, 1), Winner.P1, (1, 0)), ((1, 2), Winner.P2, (0, 1)),
                                   ((2, 2), Winner.NO_WINNER, (0, 0))):
            with self.subTest(goals=goals):
                state = GameState.new(self.config, self.rng)
                state.p1_goals, state.p2_goals = goals
                state.bot.timer = 17
                reset_round(state, self.config, self.rng)
                self.assertEqual(state.round_winner, winner)
                self.assertEqual((state.p1_wins, state.p2_wins), wins)
                self.assertEqual((state.p1_goals, state.p2_goals, state.round_count), (0, 0, 1))
                self.assertEqual(state.round_time, 60)
                self.assertEqual((state.player1.position.y, state.player2.position.y), (470, 470))
                self.assertEqual(state.bot.timer, 17)

    def test_player_clamps_at_both_edges(self):
        for y, action, expected in ((0, -1, 0), (940, 1, 940)):
            self.state.player1.position.y = F32(y)
            self.state.player1.dx = action
            update(self.state, self.config, self.rng)
            self.assertEqual(self.state.player1.position.y, expected)

    def test_bot_prediction_includes_wall_reflections(self):
        self.state.ball.position = Point(F32(500), F32(900))
        self.state.ball.dx = self.state.ball.dy = F32(1)
        self.assertEqual(Bot.predict_ball_y(self.config, self.state.player2, self.state.ball), 615)
        self.state.ball.dx = F32(0)
        self.assertEqual(Bot.predict_ball_y(self.config, self.state.player2, self.state.ball), 900)

    def test_input_order_and_bounds(self):
        handle_input(self.state, {"up"}, {"right_shift", "w", "s"})
        self.assertEqual(self.state.sim_speed, 1006)
        self.assertEqual(self.state.player1.dx, 1)
        handle_input(self.state, {"down"}, {"right_shift"})
        self.assertEqual(self.state.sim_speed, 1)
        handle_input(self.state, {"right", "up"}, {"right_shift"})
        self.assertEqual(self.state.sim_speed, 275000)
        handle_input(self.state, {"up"}, {"right_shift"})
        self.assertEqual(self.state.sim_speed, 275000)
        handle_input(self.state, {"left"}, {"right_shift", "s"})
        self.assertEqual(self.state.sim_speed, 1)
        self.assertEqual(self.state.player1.dx, -1)
        handle_input(self.state, {"space"}, set())
        self.assertTrue(self.state.is_paused)
        handle_input(self.state, {"space"}, set())
        self.assertFalse(self.state.is_paused)

    def test_only_right_shift_accelerates(self):
        handle_input(self.state, {"up", "down"}, {"left_shift"})
        self.assertEqual(self.state.sim_speed, 6)

    def test_raster_hides_waiting_ball_and_preserves_tail(self):
        buffer = [99.0] * 1025
        rasterize(self.state, self.config, buffer, False)
        self.assertNotIn(1.0, buffer[:1024])
        self.assertEqual(buffer[1024], 99.0)
        rasterize(self.state, self.config, buffer, True)
        self.assertIn(1.0, buffer[:1024])

    def test_sensors_separate_on_off_and_average(self):
        config = GameConfig(raster_size=2)
        on, off = [99.0, 99.0], [99.0, 99.0]
        raster_to_sense(PopolaConfig(), config, [1.0, 0.0, 0.75, 0.0], [0.0, 1.0, 0.0, 0.75], on, off)
        self.assertEqual(on, [0.4375, 99.0])
        self.assertEqual(off, [0.4375, 99.0])

    def test_fixed_timestep_and_speed(self):
        simulation = Simulation()
        simulation.advance(0.01)
        self.assertEqual(simulation.state.round_time, 60.0)
        simulation.advance(0.01)
        self.assertEqual(simulation.state.round_time, 59.98)
        simulation.state.sim_speed = 5.0
        simulation.advance(0.02)
        self.assertAlmostEqual(simulation.state.round_time, 59.88)

    def test_pause_clears_motion_sensors_on_next_tick(self):
        simulation = Simulation()
        simulation.state.player1.dx = 1
        simulation.tick()
        self.assertGreater(sum(simulation.sense_buffer_on), 0)
        simulation.state.is_paused = True
        simulation.tick()
        self.assertEqual(sum(simulation.sense_buffer_on) + sum(simulation.sense_buffer_off), 0)


class RenderingTests(unittest.TestCase):
    def setUp(self):
        pygame.display.init()
        pygame.freetype.init()

    def tearDown(self):
        pygame.quit()

    def test_render_at_multiple_sizes_and_pause(self):
        simulation = Simulation()
        renderer = GameRenderer.new(simulation.game_config, simulation.popola_config)
        for size in ((1920, 1080), (800, 600), (480, 800)):
            screen = pygame.display.set_mode(size, pygame.RESIZABLE)
            for paused in (False, True):
                with self.subTest(size=size, paused=paused):
                    simulation.state.is_paused = paused
                    renderer.draw(screen, simulation.state, simulation.game_config, simulation.popola_config,
                                  simulation.sense_buffer_on, simulation.sense_buffer_off, 60)
                    self.assertEqual(screen.get_at((9, size[1] - 11))[:3], WHITE)
                    self.assertEqual(screen.get_at((size[0] - 1, 0))[:3], BLACK)
            if size == (1920, 1080):
                self.assertEqual(screen.get_at((435, 540))[:3], BLUE)
                self.assertEqual(screen.get_at((1483, 540))[:3], RED)

    def test_main_processes_keys_focus_loss_and_exit(self):
        simulation = Simulation()
        events = [
            [pygame.event.Event(pygame.KEYDOWN, key=pygame.K_w)],
            [pygame.event.Event(pygame.KEYDOWN, key=pygame.K_SPACE)],
            [pygame.event.Event(pygame.KEYUP, key=pygame.K_SPACE)],
            [pygame.event.Event(pygame.KEYDOWN, key=pygame.K_SPACE)],
            [pygame.event.Event(pygame.WINDOWFOCUSLOST)],
            [pygame.event.Event(pygame.QUIT)],
        ]
        with patch("p0p0l4.__main__.Simulation", return_value=simulation), patch("pygame.event.get", side_effect=events):
            main()
        self.assertFalse(simulation.state.is_running())
        self.assertFalse(simulation.state.is_paused)
        self.assertEqual(simulation.state.player1.dx, 0)

    def test_original_camera_orientation(self):
        simulation = Simulation()
        screen = pygame.display.set_mode((1920, 1080))
        simulation.state.ball = Ball(F32(1), F32(0), F32(8), Point(F32(500), F32(100)), 0.0)
        simulation.state.player1.dx = 1
        simulation.tick()
        renderer = GameRenderer.new(simulation.game_config, simulation.popola_config)
        renderer.draw(screen, simulation.state, simulation.game_config, simulation.popola_config,
                      simulation.sense_buffer_on, simulation.sense_buffer_off, 60)
        self.assertEqual(screen.get_at((435, 495))[:3], BLUE)
        self.assertEqual(screen.get_at((435, 570))[:3], BLACK)
        self.assertEqual(screen.get_at((969, 972))[:3], WHITE)


if __name__ == "__main__":
    unittest.main()
