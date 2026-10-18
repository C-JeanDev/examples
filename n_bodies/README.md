# N-Bodies Simulation & Visualization

A high-performance $N$-body physics simulation built in **Rust** using the **Bevy** engine for 3D visualization.

## 📌 Features

- **Gravitational Physics Model**: Efficient calculation of mutual gravitational attraction between $N$ bodies.
- **Bevy Visualization Framework**: Real-time 3D rendering and camera controls powered by Bevy.
- **Modular Architecture**: Clean separation between the core simulation logic (`model`) and the rendering setup (`visualization`).

---

# How to run

- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
