mod bot;
mod config;
mod input;
mod physics;
mod raster;
mod render;
mod simulation;
mod state;

use rand::{SeedableRng, rngs::StdRng};

pub use config::GameConfig;
pub use render::GameRenderer;
pub use state::GameState;

pub use input::handle_input;
pub use raster::rasterize;
pub use simulation::update;

pub fn new_rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}
