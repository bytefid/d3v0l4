# p0p0l4 (popola)

Python port of the Pong environment with ON/OFF motion sensors.
Requires Python 3.11 or newer. Rust and Cargo are not needed.

## Run

```sh
python -m venv .venv
# macOS / Linux:
source .venv/bin/activate
# Windows PowerShell:
# .venv\Scripts\Activate.ps1
python -m pip install .
popola
```

You can also run `python -m p0p0l4`, or `python src/main.py` from the checkout
after installing the dependency with `python -m pip install .`.

## Controls

| Key | Action |
| --- | --- |
| W / S | Move player 1 up / down |
| Space | Pause / resume |
| Up / Down | Change simulation speed by 5 (range 1–275,000) |
| Right Shift + Up / Down | Apply both the 5 and 1,000 speed changes |
| Right Shift + Left / Right | Set speed to 1 / 275,000 |
| Close window | Exit |

The resizable 1920×1080 window keeps the square playfield centered. The HUD,
paddle colors, net, pause overlay, and 16×16 red/green sensor view retain the
original layout and ProggyClean font. The sensor view uses nearest-neighbor
scaling: green shows increasing pixel brightness, red shows decreasing brightness.
The original negative-Y camera flips the playfield and sensor texture vertically;
the sensor panel is at the bottom left, while the HUD uses screen coordinates.

## Behavior

Configuration lives in `src/p0p0l4/game/config.py` and
`src/p0p0l4/popola/config.py`. The game retains the original 50 Hz fixed timestep,
60-second rounds, two-second serve delay, paddle collisions, ball acceleration,
bot reaction interval, target prediction and error. Velocities are per simulation
tick, as in the Rust implementation. Pausing freezes game state; input and
per-tick sensor processing keep the original ordering.

`rng.py` implements the original PCG-seeded ChaCha12 generator and rand 0.10.3
float sampling in Python. `numeric.py` preserves single-precision game
arithmetic and uses the platform's `sinf`/`cosf`; timers remain double precision.
As with the original, native trigonometry can differ slightly between platforms.

Rendering uses pygame-ce/SDL in place of macroquad/Metal. Font rasterization
and window refresh timing can differ between backends. The speed controls keep
their original range, but achievable simulation throughput depends on hardware
and Python is slower at extreme acceleration.

## Test

```sh
python -m unittest discover -s tests -v
```

The Rust reference fixture comes from commit `c240697`, built on macOS ARM64
with the original Cargo.lock. It records four seeds across 12,000 ticks each,
including scripted movement and pauses. Tests compare game state, scores,
raster buffers and ON/OFF sensor buffers against the reference.
The full bitwise trajectory fixture runs on macOS ARM64, where its native
trigonometry was recorded. RNG/sampling parity and behavior/rendering tests run
on every platform.

## Build a standalone executable

```sh
python -m pip install ".[build]"
python -m PyInstaller --noconfirm --clean --onefile --name popola --paths src --collect-data p0p0l4 src/main.py
```

The executable is written to `dist/`. Build on the target OS; GitHub Actions
builds macOS ARM64, macOS Intel, Linux x86_64 and Windows x86_64 artifacts.
The bundled font's license is in `src/p0p0l4/assets/ProggyClean-LICENSE.txt`.
