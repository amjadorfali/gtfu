#[cfg(target_os = "macos")]
use objc2::MainThreadMarker;
#[cfg(target_os = "macos")]
use objc2_app_kit::NSWindowCollectionBehavior;
use std::num::NonZeroU32;
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::WindowLevel;

use super::winit_app;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CustomEvent {
    CLOSE,
    REDRAW,
}

pub fn get_event_loop() -> (EventLoop<CustomEvent>, EventLoopProxy<CustomEvent>) {
    #[cfg(target_os = "macos")]
    use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

    let mut event_loop_builder = EventLoop::<CustomEvent>::with_user_event();

    #[cfg(target_os = "macos")]
    event_loop_builder.with_activation_policy(ActivationPolicy::Accessory);

    let event_loop = event_loop_builder.build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();

    (event_loop, event_loop_proxy)
}

pub(crate) fn get_app(
    event_loop_proxy: EventLoopProxy<CustomEvent>,
) -> (impl ApplicationHandler<CustomEvent> + 'static) {
    winit_app::WinitAppBuilder::with_init(
        |elwt| {
            let window = winit_app::make_window(elwt, |w| {
                w.with_decorations(false)
                    .with_window_level(WindowLevel::AlwaysOnTop)
            });

            let context = softbuffer::Context::new(window.clone()).unwrap();

            #[cfg(target_os = "macos")]
            {
                use objc2::rc::Retained;
                use objc2_app_kit::NSView;
                use winit::raw_window_handle::RawWindowHandle;

                match window.window_handle().unwrap().as_raw() {
                    RawWindowHandle::AppKit(handle) => {
                        assert!(
                            MainThreadMarker::new().is_some(),
                            "can only access AppKit handles on the main thread"
                        );
                        let ns_view = handle.ns_view.as_ptr();
                        unsafe {
                            let ns_view: Retained<NSView> =
                                { Retained::retain(ns_view.cast()) }.unwrap();
                            let ns_window = ns_view
                                .window()
                                .expect("view was not installed in a window");

                            ns_window.setCollectionBehavior(
                                NSWindowCollectionBehavior::CanJoinAllSpaces,
                            );
                        }
                    }
                    handle => unreachable!("unknown handle {handle:?} for platform"),
                }
            }

            (window, context)
        },
        |_elwt, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(move |(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("Resized fired before Resumed or after Suspended");
                    return;
                };

                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    surface.resize(width, height).unwrap();
                }
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = window.inner_size();
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

            Event::WindowEvent {
                window_id: _,
                event: _,
            } => {}

            Event::AboutToWait => {}

            Event::UserEvent(CustomEvent::REDRAW) => {
                window.request_redraw();
            }

            unhandld => {
                println!("unhandled event, {:?}", unhandld);
            }
        }
    })
}
