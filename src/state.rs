use std::sync::Mutex;

use crate::service::TabService;

static SERVICE: Mutex<Option<TabService>> = Mutex::new(None);

pub fn install(service: TabService) {
    replace(Some(service));
}

pub fn clear() {
    replace(None);
}

pub fn with_service<R>(action: impl FnOnce(&mut TabService) -> R) -> Option<R> {
    let mut guard = SERVICE.lock().ok()?;
    guard.as_mut().map(action)
}

fn replace(service: Option<TabService>) {
    match SERVICE.lock() {
        Ok(mut guard) => *guard = service,
        Err(poisoned) => *poisoned.into_inner() = service,
    }
}
