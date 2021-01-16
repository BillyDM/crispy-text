use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use raw_gl_context::{GlConfig, GlContext};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut gl_config = GlConfig::default();
    gl_config.vsync = true;

    let context = GlContext::create(&window, gl_config).unwrap();

    context.make_current();

    gl::load_with(|s| context.get_proc_address(s));

    context.make_not_current();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                context.make_current();

                unsafe {
                    // Clear the screen
                    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                context.swap_buffers();
                context.make_not_current();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}