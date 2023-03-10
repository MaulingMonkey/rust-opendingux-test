fn render() {
    unsafe {
        gl::ClearColor(1.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

#[cfg(target_vendor = "gcw0")]
fn main() {
    use std::ptr::null_mut;

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

    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // https://www.khronos.org/registry/EGL/sdk/docs/man/html/eglIntro.xhtml
    // https://docs.rs/crate/egli/0.5.0
    let display = egli::Display::from_default_display().expect("No default display");
    display.initialize().expect("EGL failed to initialize");
    let config = *display
        .config_filter()
        .with_red_size(1)
        .with_green_size(1)
        .with_blue_size(1)
        //.with_conformant(egli::RenderableType::OPENGL_ES2)
        .choose_configs()
        .expect("Error querying suitable EGL configurations")
        .first()
        .expect("No suitable EGL configurations");

    let context = display.create_context(config).expect("Unable to create EGL context");
    let surface = display.create_window_surface(config, null_mut()).expect("Unable to create EGL surface");
    display.make_current(&surface, &surface, &context).expect("Unable to make EGL context current");

    // Give us a second to see logging
    println!("OpenGL initialized, will start rendering in 3 seconds");
    std::thread::sleep(std::time::Duration::from_secs(3));

    gl::ClearColor::load_with(|proc| egli::egl::get_proc_address(proc) as _);
    gl::Clear     ::load_with(|proc| egli::egl::get_proc_address(proc) as _);
    gl::Flush     ::load_with(|proc| egli::egl::get_proc_address(proc) as _);

    loop {
        render();
        unsafe { gl::Flush() };
        display.swap_buffers(&surface).expect("failed to swap buffers");
    }
}



#[cfg(not(target_vendor = "gcw0"))]
fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    use std::convert::TryInto;

    use raw_window_handle::HasRawWindowHandle;

    use glutin_winit::GlWindow;

    use glutin::prelude::*;
    use glutin::context::ContextAttributesBuilder;
    use glutin::display::GetGlDisplay;

    use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::{Fullscreen, WindowBuilder};

    let el = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Rust OpenDingux Test").with_fullscreen(Some(Fullscreen::Borderless(None)));
    let (window, gl_config) = glutin_winit::DisplayBuilder::new().with_window_builder(Some(window_builder)).build(&el, <_>::default(), |mut cfgs| cfgs.next().unwrap()).expect("DisplayBuilder::build");
    let window = window.expect("window");
    let gl_surface = unsafe { gl_config.display().create_window_surface(&gl_config, &window.build_surface_attributes(<_>::default())) }.expect("create_window_surface");
    let gl_context = unsafe { gl_config.display().create_context(&gl_config, &ContextAttributesBuilder::new().build(Some(window.raw_window_handle()))) }.expect("create_context");
    let gl_context = gl_context.make_current(&gl_surface).expect("make_current");

    gl::ClearColor::load_with(|proc| gl_config.display().get_proc_address(&std::ffi::CString::new(proc).unwrap()) as _);
    gl::Clear     ::load_with(|proc| gl_config.display().get_proc_address(&std::ffi::CString::new(proc).unwrap()) as _);

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                WindowEvent::Resized(size) => {
                    if let (Ok(width), Ok(height)) = (size.width.try_into(), size.height.try_into()) {
                        gl_surface.resize(&gl_context, width, height);
                    }
                },
                WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(virtual_keycode), state, .. }, .. } => match (virtual_keycode, state) {
                    (VirtualKeyCode::Escape, _) => { *control_flow = ControlFlow::Exit },
                    _ => {},
                },
                _ => {},
            },
            Event::RedrawRequested(_) => {
                render();
                gl_surface.swap_buffers(&gl_context).expect("swap_buffers");
            },
            _ => {},
        }
    });
}
