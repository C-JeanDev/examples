use krabmaga::engine::state::State;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::location::Real2D;
use crate::model::body::Body;
use std::any::Any;

pub struct NBodyState {
    pub bodies: Vec<Body>,   
    pub next: Vec<Body>,     
    pub g: f32,
    pub dt: f32,
    pub dim: (f32, f32),
}

impl State for NBodyState {
    fn init(&mut self, schedule: &mut Schedule) {
        for body in &self.bodies {
            schedule.schedule_repeating(Box::new(*body), 0.0, 0);
        }
    }

   fn update(&mut self, step: u64) {
        self.bodies.copy_from_slice(&self.next);
        if step % 50 == 0 {
            println!("step {step}: body1 = ({}, {})", self.bodies[1].pos.x, self.bodies[1].pos.y);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state(&self) -> &(dyn State + 'static) {
        self
    }

    fn as_state_mut(&mut self) -> &mut (dyn State + 'static) {
        self
    }

    fn reset(&mut self) {
    }
}

impl NBodyState {
    pub fn new(g: f32, dt: f32, dim: (f32, f32), num_bodies: usize) -> Self {
        let mut bodies = Vec::with_capacity(num_bodies);

        let cx = dim.0 / 2.0;
        let cy = dim.1 / 2.0;
        let star_mass = 1000.0_f32;

        // max radius: to remain inside the window (with margin)
        let r_min = 40.0_f32;
        let r_max = (dim.0.min(dim.1) / 2.0) - 20.0;

        // sun centered 
        bodies.push(Body {
            id: 0,
            mass: star_mass,
            pos: Real2D { x: cx, y: cy },
            vel: Real2D { x: 0.0, y: 0.0 },
            acc: Real2D { x: 0.0, y: 0.0 },
            radius: 6.0,
        });

        // golden angle
        let golden_angle = 2.399_963_f32;
        let n = (num_bodies.max(2) - 1) as f32;

        for i in 1..num_bodies {
            let t = i as f32 / n;
            let r = r_min + (r_max - r_min) * t.sqrt();
            let a = i as f32 * golden_angle;

            let x = cx + r * a.cos();
            let y = cy + r * a.sin();

            // circular velocity 
            let v = (g * star_mass / r).sqrt();
            let vx = -a.sin() * v;
            let vy =  a.cos() * v;

            bodies.push(Body {
                id: i as u32,
                mass: 1.0,
                pos: Real2D { x, y },
                vel: Real2D { x: vx, y: vy },
                acc: Real2D { x: 0.0, y: 0.0 },
                radius: 2.0,
            });
        }

        NBodyState {
            next: bodies.clone(),
            bodies,
            g,
            dt,
            dim,
        }
    }
}