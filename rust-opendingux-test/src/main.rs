use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
#[cfg(target_vendor = "gcw0")] use glutin::{Api, GlRequest};

fn main() {
    #[cfg(target_vendor = "gcw0")]
    std::panic::set_hook(Box::new(|info|{
        if let Some(loc) = info.location() {
            eprint!("{}:{}:{}: PANIC", loc.file(), loc.line(), loc.column());
        } else {
            eprint!("PANIC");
        }

        let payload = info.payload();
        if let Some(s) = payload.downcast_ref::<&str>() {
            eprintln!(": {}", s);
        } else if let Some(s) = payload.downcast_ref::<String>() {
            eprintln!(": {}", s);
        } else {
            eprintln!();
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }));

    let el = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Rust OpenDingux Test").with_fullscreen(Some(Fullscreen::Borderless(el.available_monitors().next().expect("MonitorHandle"))));
    let context_wrapper = unsafe {
        let cb = glutin::ContextBuilder::new();
        #[cfg(target_vendor = "gcw0")] let cb = cb.with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)));
        cb.build_windowed(window_builder, &el).expect("WindowedContext").make_current().expect("ContextWrapper")
    };
    
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
