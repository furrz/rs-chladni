mod display;
mod particle;

use cgmath::prelude::*;
use cgmath::vec2;
use glium::glutin::event_loop::EventLoop;
use glium::index::PrimitiveType;
use glium::{glutin, implement_vertex, DrawParameters, Surface};
use rand::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::f32::consts::PI;

const N: f32 = 8.2;
const M: f32 = 3.8;
const MAX_PARTICLE_JITTER: f32 = 0.002;
const RESOLUTION: usize = 500;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = display::make_display(&event_loop);

    // Prepare particle grid for simulation
    let mut particles = Vec::<particle::Particle>::new();
    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            particles.push(particle::Particle {
                position: [
                    (x as f32 / RESOLUTION as f32 - 0.5) * 1.8,
                    (y as f32 / RESOLUTION as f32 - 0.5) * 1.8,
                ],
            });
        }
    }

    // Decide on simulation starting time (n seconds from now)
    let t = std::time::Instant::now() + std::time::Duration::from_secs(5);

    // Prepare point-rendering stuff
    let vb =
        glium::VertexBuffer::<particle::Particle>::empty_dynamic(&display, RESOLUTION * RESOLUTION).unwrap();
    let program = glium::Program::from_source(&display, VERTEX_SRC, FRAGMENT_SRC, None).unwrap();
    let indices = glium::index::NoIndices(PrimitiveType::Points);
    let draw_parameters = DrawParameters {
        point_size: Some(1.0),
        ..Default::default()
    };

    // Prepare RNG
    let mut rng = Xoshiro256Plus::from_entropy();

    // Run!
    display::simulate(event_loop, move || {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Only simulate and render if we've passed the target time
        if std::time::Instant::now() > t {
            for p in particles.iter_mut() {
                p.tick(&mut rng);
            }

            vb.write(&particles);

            target
                .draw(&vb, &indices, &program, &glium::uniforms::EmptyUniforms, &draw_parameters)
                .unwrap();
        }

        target.finish().unwrap();
    });
}

const FRAGMENT_SRC: &str = r#"
    #version 330 core
    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

const VERTEX_SRC: &str = r#"
    #version 330 core
    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;
