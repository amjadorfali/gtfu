struct Application {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl Application {
    fn new() -> Application {
        Self {
            window: None,
            surface: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_window_level(winit::window::WindowLevel::AlwaysOnTop),
                )
                .unwrap(),
        );
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let surface = self.surface.as_mut();
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } if window_id == self.window.as_ref().unwrap().id() => {
                let window = self.window.take();
                drop(self.surface.take());
                drop(window);
                event_loop.exit();
            }

            WindowEvent::RedrawRequested if window_id == self.window.as_ref().unwrap().id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = self.window.as_ref().unwrap().inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    let mut buffer = surface.buffer_mut().unwrap();
                    for y in 0..height.get() {
                        for x in 0..width.get() {
                            let red = x % 255;
                            let green = y % 255;
                            let blue = (x * y) % 255;
                            let index = y as usize * width.get() as usize + x as usize;
                            buffer[index] = blue | (green << 8) | (red << 16);
                        }
                    }

                    buffer.present().unwrap();
                }
            }

            WindowEvent::Destroyed => {
                println!("window destroyed");
                event_loop.exit();
            }

            WindowEvent::CloseRequested => {
                let _ = self.window.take();
            }

            _ => {}
        }
    }
}
fn run_window() {
    let mut event_loop = EventLoop::builder()
        .with_activation_policy(winit::platform::macos::ActivationPolicy::Accessory)
        .build()
        .unwrap();
    let _ = event_loop.run_app_on_demand(&mut Application::new());
}
