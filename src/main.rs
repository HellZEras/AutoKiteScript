use winapi::um::winuser::{GetKeyState, VK_XBUTTON1};
use std::{thread::sleep, time::Duration};
use enigo::*;
use std::sync::{Arc, Mutex};
use std::thread;


struct SharedState {
    toggle: bool,
}

fn is_key_pressed(virtual_key_code: i32) -> bool {
    unsafe { GetKeyState(virtual_key_code) as u16 & 0x8000 != 0 }
}
impl Default for SharedState {
    fn default() -> Self {
        Self {
            toggle: false,
        }
    }
}

fn main() {
    let shared_state = Arc::new(Mutex::new(SharedState { toggle: false }));
    let mut enigo = Enigo::new();
    let handle = thread::spawn({
        let shared_state = Arc::clone(&shared_state);
        move || {
            loop {
                if is_key_pressed(VK_XBUTTON1) {
                    let mut state = shared_state.lock().unwrap();
                    state.toggle = !state.toggle;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    });
    
    loop {
        let state = shared_state.lock().unwrap();
        let toggle_value = state.toggle;

        if toggle_value {
            println!("Toggle On");
            enigo.key_click(Key::Layout('q'));
            enigo.mouse_click(MouseButton::Left);
            enigo.mouse_click(MouseButton::Right);

        } else {
            println!("Toggle Off");
        }
        
    }
    handle.join().unwrap();

    
}