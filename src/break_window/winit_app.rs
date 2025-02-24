/// Common boilerplate for setting up a winit application.
use std::marker::PhantomData;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, NamedKey};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::window::{Window, WindowAttributes, WindowId};

use super::window::CustomEvent;

/// Run a Winit application.
#[allow(unused_mut)]
pub(crate) fn run_app(
    mut event_loop: EventLoop<CustomEvent>,
    app: &mut (impl ApplicationHandler<CustomEvent> + 'static),
    event_loop_proxy: &EventLoopProxy<CustomEvent>,
) -> EventLoop<CustomEvent> {
    // Need to trigger a redraw as the first one from OS is happening before resize, which is
    // causing the window to not have accurate surface calculations
    let _ = event_loop_proxy.send_event(CustomEvent::REDRAW);
    event_loop.run_app_on_demand(app).unwrap();
    event_loop
}

/// Create a window from a set of window attributes.
#[allow(dead_code)]
pub(crate) fn make_window(
    elwt: &ActiveEventLoop,
    f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
    let attributes = f(WindowAttributes::default());
    let window = elwt.create_window(attributes);
    Rc::new(window.unwrap())
}

/// Easily constructable winit application.
pub(crate) struct WinitApp<T, S, Init, InitSurface, Handler> {
    /// Closure to initialize `state`.
    init: Init,

    /// Closure to initialize `surface_state`.
    init_surface: InitSurface,

    /// Closure to run on window events.
    event: Handler,

    /// Contained state.
    pub state: Option<T>,

    /// Contained surface state.
    surface_state: Option<S>,
}

/// Builder that makes it so we don't have to name `T`.
pub(crate) struct WinitAppBuilder<T, S, Init, InitSurface> {
    /// Closure to initialize `state`.
    init: Init,

    /// Closure to initialize `surface_state`.
    init_surface: InitSurface,

    /// Eat the type parameter.
    _marker: PhantomData<(Option<T>, Option<S>)>,
}

impl<T, S, Init, InitSurface> WinitAppBuilder<T, S, Init, InitSurface>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
{
    /// Create with an "init" closure.
    pub(crate) fn with_init(init: Init, init_surface: InitSurface) -> Self {
        Self {
            init,
            init_surface,
            _marker: PhantomData,
        }
    }

    /// Build a new application.
    pub(crate) fn with_event_handler<F>(self, handler: F) -> WinitApp<T, S, Init, InitSurface, F>
    where
        F: FnMut(&mut T, Option<&mut S>, Event<CustomEvent>, &ActiveEventLoop),
    {
        WinitApp::new(self.init, self.init_surface, handler)
    }
}

impl<T, S, Init, InitSurface, Handler> WinitApp<T, S, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
    Handler: FnMut(&mut T, Option<&mut S>, Event<CustomEvent>, &ActiveEventLoop),
{
    /// Create a new application.
    pub(crate) fn new(init: Init, init_surface: InitSurface, event: Handler) -> Self {
        Self {
            init,
            init_surface,
            event,
            state: None,
            surface_state: None,
        }
    }

    pub fn drop_window(&mut self) {
        if let Some(state) = self.state.take() {
            drop(self.surface_state.take());
            drop(state);
        }
    }
}

impl<T, S, Init, InitSurface, Handler> ApplicationHandler<CustomEvent>
    for WinitApp<T, S, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
    Handler: FnMut(&mut T, Option<&mut S>, Event<CustomEvent>, &ActiveEventLoop),
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        debug_assert!(self.state.is_none());
        let mut state = (self.init)(el);
        self.surface_state = Some((self.init_surface)(el, &mut state));
        self.state = Some(state);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let surface_state = self.surface_state.take();
        debug_assert!(surface_state.is_some());
        drop(surface_state);
        println!("suspended");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                (*self).drop_window();
            }

            WindowEvent::Destroyed => {
                event_loop.exit();
                return;
            }
            _ => {}
        }

        let state = self.state.as_mut();
        let surface_state = self.surface_state.as_mut();

        if let Some(state) = state {
            (self.event)(
                state,
                surface_state,
                Event::WindowEvent { window_id, event },
                event_loop,
            );
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            (self.event)(
                state,
                self.surface_state.as_mut(),
                Event::AboutToWait,
                event_loop,
            );
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvent) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        match event {
            CustomEvent::CLOSE => {
                (*self).drop_window();
            }
            CustomEvent::REDRAW => {
                if let Some(state) = self.state.as_mut() {
                    (self.event)(
                        state,
                        self.surface_state.as_mut(),
                        Event::UserEvent(CustomEvent::REDRAW),
                        event_loop,
                    )
                }
            }
        }
    }
}
