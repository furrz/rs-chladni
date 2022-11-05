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
    // Prepare window
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Chladni Plate Simulator")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 1024));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Prepare particle grid for simulation
    let mut particles = Vec::<Particle>::new();
    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            particles.push(Particle {
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
        glium::VertexBuffer::<Particle>::empty_dynamic(&display, RESOLUTION * RESOLUTION).unwrap();
    let program = glium::Program::from_source(&display, VERTEX_SRC, FRAGMENT_SRC, None).unwrap();
    let indices = glium::index::NoIndices(PrimitiveType::Points);
    let draw_parameters = DrawParameters {
        point_size: Some(1.0),
        ..Default::default()
    };

    // Prepare RNG
    let mut rng = Xoshiro256Plus::from_entropy();

    // Run!
    simulate(event_loop, move || {
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

#[derive(Copy, Clone)]
struct Particle {
    position: [f32; 2],
}

impl Particle {
    fn tick<R>(&mut self, rng: &mut R)
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

fn simulate<T>(event_loop: EventLoop<()>, mut func: T)
    where
        T: 'static + FnMut(),
{
    event_loop.run(move |ev, _, control_flow| {
        func();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        if let glutin::event::Event::WindowEvent { event, .. } = ev {
            if event == glutin::event::WindowEvent::CloseRequested {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        }
    });
}
