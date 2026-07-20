use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn spawn(
    keys: String,
    interval_ms: u64,
    running: Arc<AtomicBool>,
) -> Result<JoinHandle<()>, String> {
    if keys.is_empty() {
        return Err("no keys provided".into());
    }

    running.store(true, Ordering::SeqCst);

    let handle = thread::spawn(move || {
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(_) => {
                running.store(false, Ordering::SeqCst);
                return;
            }
        };

        while running.load(Ordering::SeqCst) {
            for ch in keys.chars() {
                if !running.load(Ordering::SeqCst) {
                    break;
                }
                let _ = enigo.key(Key::Unicode(ch), Direction::Click);
                thread::sleep(Duration::from_millis(20));
            }
            thread::sleep(Duration::from_millis(interval_ms.max(1)));
        }
    });

    Ok(handle)
}
