use crate::state::KeyBind;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn spawn(
    binds: Vec<KeyBind>,
    running: Arc<AtomicBool>,
) -> Result<Vec<JoinHandle<()>>, String> {
    if binds.is_empty() {
        return Err("no keys provided".into());
    }
    for bind in &binds {
        if bind.key.chars().next().is_none() {
            return Err("empty key in bind list".into());
        }
    }

    running.store(true, Ordering::SeqCst);

    let handles = binds
        .into_iter()
        .map(|bind| {
            let running = running.clone();
            let ch = bind.key.chars().next().unwrap();
            let interval_ms = bind.interval_ms.max(1);

            thread::spawn(move || {
                let mut enigo = match Enigo::new(&Settings::default()) {
                    Ok(e) => e,
                    Err(_) => return,
                };

                while running.load(Ordering::SeqCst) {
                    let _ = enigo.key(Key::Unicode(ch), Direction::Click);
                    thread::sleep(Duration::from_millis(interval_ms));
                }
            })
        })
        .collect();

    Ok(handles)
}
