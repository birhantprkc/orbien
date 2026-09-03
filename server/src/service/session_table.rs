use crate::control::Control;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};

pub(crate) struct SessionEntry {
    pub control: Arc<Control>,
    pub gate: Arc<Mutex<()>>,
}

pub(crate) type SessionMap = HashMap<String, SessionEntry>;

pub(super) async fn swap_in_locked(
    controls: &Mutex<SessionMap>,
    session_id: &str,
    control: Arc<Control>,
) -> (OwnedMutexGuard<()>, Option<Arc<Control>>) {
    loop {
        let peeked_gate = {
            let map = controls.lock().await;
            map.get(session_id).map(|e| Arc::clone(&e.gate))
        };

        let gate = peeked_gate.unwrap_or_else(|| Arc::new(Mutex::new(())));
        let guard = Arc::clone(&gate).lock_owned().await;

        let mut map = controls.lock().await;
        if let Some(entry) = map.get(session_id) {
            if !Arc::ptr_eq(&entry.gate, &gate) {
                drop(map);
                drop(guard);
                continue;
            }
        }

        let previous = map.insert(
            session_id.to_string(),
            SessionEntry {
                control: Arc::clone(&control),
                gate: Arc::clone(&gate),
            },
        );
        drop(map);
        return (guard, previous.map(|e| e.control));
    }
}

pub(super) async fn remove_if_current(
    controls: &Mutex<SessionMap>,
    session_id: &str,
    control: &Arc<Control>,
) -> bool {
    let gate = {
        let map = controls.lock().await;
        match map.get(session_id) {
            Some(entry) if Arc::ptr_eq(&entry.control, control) => Arc::clone(&entry.gate),
            _ => return false,
        }
    };

    let _guard = gate.lock_owned().await;
    let mut map = controls.lock().await;
    if map
        .get(session_id)
        .map(|e| Arc::ptr_eq(&e.control, control))
        .unwrap_or(false)
    {
        map.remove(session_id);
        true
    } else {
        false
    }
}

pub(super) async fn lookup_accepting(
    controls: &Mutex<SessionMap>,
    session_id: &str,
) -> Option<Arc<Control>> {
    let (gate, candidate) = {
        let map = controls.lock().await;
        let entry = map.get(session_id)?;
        (Arc::clone(&entry.gate), Arc::clone(&entry.control))
    };

    let _guard = gate.lock_owned().await;
    let map = controls.lock().await;
    let entry = map.get(session_id)?;
    if Arc::ptr_eq(&entry.control, &candidate) && candidate.is_accepting_data() {
        Some(candidate)
    } else {
        None
    }
}
