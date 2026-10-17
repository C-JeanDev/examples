use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::engine::location::Real2D;
use crate::model::state::NBodyState;

#[derive(Clone, Copy)]
pub struct Body {
    pub id: u32,
    pub mass: f32,
    pub pos: Real2D,
    pub vel: Real2D,
    pub acc: Real2D,
    pub radius: f32,
}

impl Agent for Body {
    fn step(&mut self, state: &mut dyn State) {

        // Downcast
        let state = state.as_any_mut()
            .downcast_mut::<NBodyState>()
            .expect("stato non NBodyState");

        let mut total_force_x = 0.0;
        let mut total_force_y = 0.0;

        // Force exerted by all bodies
        for other in &state.bodies {
            if other.id == self.id {
                continue;
            }

            let dx = other.pos.x - self.pos.x;
            let dy = other.pos.y - self.pos.y;
            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt();

            if dist < 1e-6 {
                continue;
            }

            // Gravitational law F = G * (m1 * m2) / r^2
            // F_x = F * (dx / dist) -> F_x = G * m1 * m2 * dx / (dist^3)
            let force_magnitude = (state.g * self.mass * other.mass) / (dist_sq * dist);
            
            total_force_x += force_magnitude * dx;
            total_force_y += force_magnitude * dy;

            state.next[self.id as usize] = *self;
        }

        // Acceleration (F = m * a -> a = F / m)
        if self.mass > 0.0 {
            self.acc.x = total_force_x / self.mass;
            self.acc.y = total_force_y / self.mass;
        }

        // Update velocity and position (Euler Method)
        // v_{t+1} = v_t + a * dt
        self.vel.x += self.acc.x * state.dt;
        self.vel.y += self.acc.y * state.dt;

        // p_{t+1} = p_t + v_{t+1} * dt
        self.pos.x += self.vel.x * state.dt;
        self.pos.y += self.vel.y * state.dt;
    }
}