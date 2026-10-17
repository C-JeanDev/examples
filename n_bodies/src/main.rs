mod model;
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let state = model::state::NBodyState::new(1.0, 0.05, (800.0, 600.0), 100);
    simulate!(state, 1000, 1);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    use crate::model::state::NBodyState;
    use krabmaga::bevy::prelude::Color;
    use krabmaga::visualization::visualization::Visualization;
    use crate::visualization::vis_state::NBodyVis;

    let dim = (800.0, 600.0);
    let state = NBodyState::new(100.0, 0.05, dim, 3);

    Visualization::default()
        .with_window_dimensions(800., 600.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::srgb(0.02, 0.02, 0.08))
        .with_name("N-Body")
        .start::<NBodyVis, NBodyState>(NBodyVis, state);
}