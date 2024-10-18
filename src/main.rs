use enigo::{Enigo, Mouse, Settings, Coordinate};
use rdev::{listen, Event, EventType, Key};
use std::{ error, sync::{atomic::{AtomicBool, AtomicI32, Ordering}, Arc}, thread, time};

fn main() {
    println!("Lock Mouse Pointer");
    println!("Usage: Press 'F6' to activate/deactivate the mouse pointer");

    let is_activated = Arc::new(AtomicBool::new(false));
    let is_activated_thread = Arc::clone(&is_activated);

    let mouse_x = Arc::new(AtomicI32::new(0));
    let mouse_x_thread = Arc::clone(&mouse_x);

    let mouse_y = Arc::new(AtomicI32::new(0));
    let mouse_y_thread = Arc::clone(&mouse_y);

    thread::spawn(move || {
        let settings = Settings::default();
        let mut enigo = Enigo::new(&settings).expect("Failed to create Enigo instance");
;

        loop {
            // If the cursor lock is activated
            if is_activated_thread.load(Ordering::Relaxed) {
                // Move the mouse to the stored X and Y coordinates
                enigo.move_mouse(mouse_x_thread.load(Ordering::Relaxed), mouse_y_thread.load(Ordering::Relaxed), Coordinate::Abs);
                // Sleep for 3 milliseconds before checking again (to avoid excessive CPU usage)
                thread::sleep(time::Duration::from_millis(3));
            }
        }

    });

    if let Err(error) = listen(move |event| {
        if event.event_type == EventType::KeyPress(Key::F6) {
            is_activated.store(!is_activated.load(Ordering::Relaxed), Ordering::Relaxed);

            if is_activated.load(Ordering::Relaxed) {
                println!("Cursor Lock Activated");
            } else {
                println!("Cursor Lock Deactivated");
            }
        }

        let settings = Settings::default();
        let mut enigo = Enigo::new(&settings);
        let mouse_position = enigo.expect("REASON").location().expect("Failed to get mouse location");

        mouse_x.store(mouse_position.0, Ordering::Relaxed);
        mouse_y.store(mouse_position.1, Ordering::Relaxed);

    }) {
        println!("Error {:?}", error)
    }


}
