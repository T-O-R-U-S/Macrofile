use enigo::Key::{self, *};

pub fn input_to_keycode(input: String) -> Key {
    match input.as_str() {
        "f1" => F1,
        "f2" => F2,
        "f3" => F3,
        "f4" => F4,
        "f5" => F5,
        "f6" => F6,
        "f7" => F7,
        "f8" => F8,
        "f9" => F9,
        "f10" => F10,
        "f11" => F11,
        "f12" => F12,
        "caps" => CapsLock,
        "ret" => Return,
        "home" => Home,
        "end" => End,
        "tab" => Tab,
        "esc" => Escape,
        "del" => Delete,
        "ctrl" => Control,
        "super" => Meta,
        "pgup" => PageUp,
        "pgdn" => PageDown,
        "d-arr" => DownArrow,
        "u-arr" => UpArrow,
        "l-arr" => LeftArrow,
        "r-arr" => RightArrow,
        "alt" => Alt,
        "shft" => Shift,
        "spc" => Space,
        "bks" => Backspace,
        // Ugly :(
        chr if chr.chars().nth(0).unwrap().is_alphabetic() => {
            Key::Layout(chr.chars().nth(0).unwrap())
        }
        any => panic!("Unexpected string: {:?}. (Do not use capitals!)", any),
    }
}
