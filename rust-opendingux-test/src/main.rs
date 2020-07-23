use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};

fn main() {
    let el = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Rust OpenDingux Test").with_fullscreen(Some(Fullscreen::Borderless(el.available_monitors().next().expect("MonitorHandle"))));
    let context_wrapper = unsafe { glutin::ContextBuilder::new().build_windowed(window_builder, &el).expect("WindowedContext").make_current().expect("ContextWrapper") };
    
    gl::load_with(|proc| context_wrapper.get_proc_address(proc) as *const _);
    
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                WindowEvent::Resized(size) => context_wrapper.resize(size),
                WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(virtual_keycode), state, .. }, .. } => match (virtual_keycode, state) {
                    (VirtualKeyCode::Escape, _) => { *control_flow = ControlFlow::Exit },
                    _ => {},
                },
                _ => {},
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                context_wrapper.swap_buffers().expect("swap_buffers");
            },
            _ => {},
        }
    });
}
