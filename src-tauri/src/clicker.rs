use enigo::{Button, Direction, Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn spawn(
    speed_ms: u64,
    button: String,
    running: Arc<AtomicBool>,
) -> Result<JoinHandle<()>, String> {
    let btn = match button.as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        other => return Err(format!("invalid mouse button: {other}")),
    };

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
            let _ = enigo.button(btn, Direction::Click);
            thread::sleep(Duration::from_millis(speed_ms.max(1)));
        }
    });

    Ok(handle)
}
