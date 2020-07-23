fn render() {
    unsafe {
        gl::ClearColor(1.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

#[cfg(target_vendor = "gcw0")]
fn main() {
    #[link(name = "EGL")]
    #[link(name = "GLESv2")]
    extern {}

    const ATTRIBUTE_LIST : &[egl::EGLint] = &[
        egl::EGL_RED_SIZE, 1,
        egl::EGL_GREEN_SIZE, 1,
        egl::EGL_BLUE_SIZE, 1,
        egl::EGL_NONE,
    ];

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

    // https://www.khronos.org/registry/EGL/sdk/docs/man/html/eglIntro.xhtml
    let display = egl::get_display(egl::EGL_DEFAULT_DISPLAY).expect("No default display");
    // UNSOUND?  `display_id` is a raw pointer.

    let (mut major, mut minor) = (0, 0);
    assert!(egl::initialize(display, &mut major, &mut minor), "EGL failed to initialize");
    // UNSOUND?  `display` is a raw pointer.

    let config = egl::choose_config(display, ATTRIBUTE_LIST, 1).expect("No matching EGL configuration");
    // UNSOUND AS HECK API WITH TOTALLY WRONG TYPE SIGNATURE?
    // 1. display is a raw pointer.
    // 2. no guarantee that ATTRIBUTE_LIST is terminated by EGL_NONE, which the underlying C fn requires
    // 3. config_size should correspond to the size of a configs array.  Instead, internally this has a single config always for trivial buffer overflows:
    // ```no_run
    // let mut config: EGLConfig = ptr::null_mut();
    // ```
    // Despite these issues, this function isn't marked unsafe!

    let context = egl::create_context(display, config, egl::EGL_NO_CONTEXT, &[]).expect("Unable to create EGL context");
    // UNSOUND (pointers)

    let surface = egl::create_window_surface(display, config, std::ptr::null_mut(), &[]).expect("Unable to create EGL surface");
    // UNSOUND (pointers)

    assert!(egl::make_current(display, surface, surface, context), "Unable to make EGL context current");
    // UNSOUND (pointers)

    // Give us a second to see logging
    println!("OpenGL initialized, will start rendering in 3 seconds");
    std::thread::sleep(std::time::Duration::from_secs(3));

    gl::ClearColor::load_with(|proc| egl::get_proc_address(proc) as _);
    gl::Clear     ::load_with(|proc| egl::get_proc_address(proc) as _);
    gl::Flush     ::load_with(|proc| egl::get_proc_address(proc) as _);

    loop {
        render();
        unsafe { gl::Flush() };
        egl::swap_buffers(display, surface);
    }
}



#[cfg(not(target_vendor = "gcw0"))]
fn main() {
    use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
    use glutin::event_loop::{ControlFlow, EventLoop};
    use glutin::window::{Fullscreen, WindowBuilder};

    let el = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Rust OpenDingux Test").with_fullscreen(Some(Fullscreen::Borderless(el.available_monitors().next().expect("MonitorHandle"))));
    let context_wrapper = unsafe { glutin::ContextBuilder::new().build_windowed(window_builder, &el).expect("WindowedContext").make_current().expect("ContextWrapper") };
    
    gl::ClearColor::load_with(|proc| context_wrapper.get_proc_address(proc) as _);
    gl::Clear     ::load_with(|proc| context_wrapper.get_proc_address(proc) as _);
    
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
                render();
                context_wrapper.swap_buffers().expect("swap_buffers");
            },
            _ => {},
        }
    });
}
