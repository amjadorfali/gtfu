use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wayland_client::{
    protocol::wl_registry, protocol::wl_seat::WlSeat, Connection, Dispatch, QueueHandle,
};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::{Event as NotificationEvent, ExtIdleNotificationV1},
    ext_idle_notifier_v1::{Event as NotifierEvent, ExtIdleNotifierV1},
};

struct IdleState {
    last_activity: AtomicU64,
}

struct AppState {
    idle_state: Arc<IdleState>,
    seats: Vec<WlSeat>,
    notifier: Option<ExtIdleNotifierV1>,
    notifications: Vec<ExtIdleNotificationV1>,
}

impl AppState {
    fn new(idle_state: Arc<IdleState>) -> Self {
        Self {
            idle_state,
            seats: Vec::new(),
            notifier: None,
            notifications: Vec::new(),
        }
    }
}

impl Dispatch<WlSeat, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &WlSeat,
        _: <WlSeat as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _globals: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_seat" => {
                    let seat = registry.bind(name, version, qh, ());
                    state.seats.push(seat);
                }
                "ext_idle_notifier_v1" => {
                    let notifier = registry.bind(name, version, qh, ());
                    state.notifier = Some(notifier);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<ExtIdleNotifierV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &ExtIdleNotifierV1,
        _: NotifierEvent,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // No events we need to handle for the notifier itself
    }
}

impl Dispatch<ExtIdleNotificationV1, ()> for AppState {
    fn event(
        state: &mut Self,
        _: &ExtIdleNotificationV1,
        event: NotificationEvent,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            NotificationEvent::Idled => {
                state
                    .idle_state
                    .last_activity
                    .store(Instant::now().elapsed().as_secs(), Ordering::Relaxed);
            }
            NotificationEvent::Resumed => {
                state.idle_state.last_activity.store(0, Ordering::Relaxed);
            }
            _ => {}
        }
    }
}

lazy_static::lazy_static! {
    static ref IDLE_STATE: Arc<IdleState> = Arc::new(IdleState {
        last_activity: AtomicU64::new(0)
    });
    static ref EVENT_THREAD: Mutex<Option<std::thread::JoinHandle<()>>> = Mutex::new(None);
}

pub fn initialize() {
    if !EVENT_THREAD.lock().unwrap().is_none() {
        return;
    }
    let conn = Connection::connect_to_env().unwrap();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    conn.display().get_registry(&qh, ());

    let mut state = AppState::new(IDLE_STATE.clone());

    // Initial roundtrip to get registry globals
    event_queue.roundtrip(&mut state).unwrap();

    // Create notifications for existing seats
    if let Some(notifier) = &state.notifier {
        for seat in &state.seats {
            let notification = notifier.get_idle_notification(1000, seat, &qh, ());
            state.notifications.push(notification);
        }
    }

    // Spawn event thread
    *EVENT_THREAD.lock().unwrap() = Some(std::thread::spawn(move || loop {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }));
}

pub fn get_idle_time() -> std::time::Duration {
    let current = Instant::now().elapsed().as_secs();
    let last = IDLE_STATE.last_activity.load(Ordering::Relaxed);
    std::time::Duration::from_secs(current.saturating_sub(last))
}
