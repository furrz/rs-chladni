use std::f32::consts::PI;
use cgmath::{InnerSpace, vec2};
use glium::implement_vertex;
use rand::Rng;
use crate::{M, MAX_PARTICLE_JITTER, N};

#[derive(Copy, Clone)]
pub struct Particle {
    pub position: [f32; 2],
}

impl Particle {
    pub fn tick<R>(&mut self, rng: &mut R)
        where
            R: Rng,
    {
        // Calculate intensity of the vibration of the plate at the current location.
        let wave_strength = (N * PI * self.position[0]).sin() * (M * PI * self.position[1]).sin()
            - (M * PI * self.position[0]).sin() * (N * PI * self.position[1]).sin();

        let jitter_scale = rng.gen_range(0.0..1.0);

        // Jitter in a random direction
        let delta = vec2(rng.gen_range(0.0..1.0f32), rng.gen_range(0.0..1.0f32)).normalize()
            * wave_strength
            * MAX_PARTICLE_JITTER
            * jitter_scale;

        self.position[0] += delta.x;
        self.position[1] += delta.y;
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            position: [0.0, 0.0],
        }
    }
}

implement_vertex!(Particle, position);
