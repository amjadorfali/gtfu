use objc2::MainThreadMarker;
use objc2_app_kit::NSWindowCollectionBehavior;
use std::num::NonZeroU32;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::WindowLevel;

use crate::winit_app;

#[derive(Debug, PartialEq)]
pub enum CustomEvent {
    CLOSE,
}
pub fn init_app() -> (impl FnOnce(), EventLoopProxy<CustomEvent>) {
    #[cfg(target_os = "macos")]
    use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

    let mut event_loop_builder = EventLoop::<CustomEvent>::with_user_event();

    #[cfg(target_os = "macos")]
    event_loop_builder.with_activation_policy(ActivationPolicy::Accessory);

    let event_loop = event_loop_builder.build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();

    (
        || {
            entry(event_loop);
        },
        event_loop_proxy,
    )
}

pub(crate) fn entry(event_loop: EventLoop<CustomEvent>) {
    let app = winit_app::WinitAppBuilder::with_init(
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
    .with_event_handler(|(window, _context), surface, event, elwt| {
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
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }

            Event::UserEvent(CustomEvent::CLOSE) => {
                println!("should exit");
                elwt.exit();
            }
            Event::WindowEvent {
                window_id: _,
                event: _,
            } => {}
            Event::AboutToWait => {}
            unhandld => {
                println!("unhandled event, {:?}", unhandld);
            }
        }
    });

    winit_app::run_app(event_loop, app);
}
