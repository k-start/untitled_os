use crate::print;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, ScancodeSet1};
use spin::Mutex;

lazy_static! {
    static ref KEYBOARD: Mutex<pc_keyboard::Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        pc_keyboard::Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub fn key_pressed() {
    let mut keyboard = KEYBOARD.lock();

    let scancode = crate::inb(0x60);
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                // DecodedKey::RawKey(key) => print!("{:?}", key),
                DecodedKey::RawKey(_) => {}
            }
        }
    }
}
