from .state import PlayerNum


def handle_input(state, pressed, down):
    """Use key names to keep input logic testable without a display."""
    if "space" in pressed:
        state.is_paused = not state.is_paused
    if "up" in pressed:
        state.sim_speed = min(state.sim_speed + 5.0, 275_000.0)
    elif "down" in pressed:
        state.sim_speed = max(state.sim_speed - 5.0, 1.0)
    if "up" in pressed and "right_shift" in down:
        state.sim_speed = min(state.sim_speed + 1000.0, 275_000.0)
    elif "down" in pressed and "right_shift" in down:
        state.sim_speed = max(state.sim_speed - 1000.0, 1.0)
    if "left" in pressed and "right_shift" in down:
        state.sim_speed = 1.0
    elif "right" in pressed and "right_shift" in down:
        state.sim_speed = 275_000.0
    action = 1 if "w" in down else -1 if "s" in down else 0
    state.player_action(PlayerNum.P1, action)
