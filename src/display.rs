use glium::glutin;

pub fn make_display<T>(event_loop: &glutin::event_loop::EventLoop<T>) -> glium::Display
    where T: 'static
{
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Chladni Plate Simulator")
        .with_inner_size(glutin::dpi::PhysicalSize::new(1500, 1500));
    let cb = glutin::ContextBuilder::new();
    return glium::Display::new(wb, cb, &event_loop).unwrap();
}

pub fn simulate<T, Y>(event_loop: glutin::event_loop::EventLoop<T>, mut func: Y)
    where T: 'static, Y: 'static + FnMut()
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