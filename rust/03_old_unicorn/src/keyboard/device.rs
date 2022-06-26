use libc::{c_int, input_event};
use nix::ioctl_write_ptr;
use std::fs::File;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::{collections::HashMap, mem};

use crate::error::Error;

ioctl_write_ptr!(eviocgrab, b'E', 0x90, c_int);

const SIZE_OF_INPUT_EVENT: usize = mem::size_of::<input_event>();

pub struct InputDevice {
    device_file: File,
    buf: [u8; SIZE_OF_INPUT_EVENT],
}

impl InputDevice {
    pub fn open(device_file: &str) -> Result<Self, Error> {
        let device_file = File::open(device_file)?;
        Ok(InputDevice {
            device_file: device_file,
            buf: [0u8; SIZE_OF_INPUT_EVENT],
        })
    }

    pub fn read_event(&mut self) -> Result<input_event, Error> {
        let num_bytes = self.device_file.read(&mut self.buf)?;
        if num_bytes != SIZE_OF_INPUT_EVENT {
            return Err(Error::ShortRead);
        }
        let event: input_event = unsafe { mem::transmute(self.buf) };
        Ok(event)
    }

    pub fn grab(&mut self) -> Result<(), Error> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 1 as *const c_int)?;
        }
        Ok(())
    }

    pub fn release(&mut self) -> Result<(), Error> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 0 as *const c_int)?;
        }
        Ok(())
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        self.release().ok(); // ignore any errors here, what could we do anyhow?
    }
}

pub fn key_map() -> HashMap<u16, &'static str> {
    [
        // generated like:
        // grep -o 'KEY_[^ :;]*' ~/.cargo/registry/src/github.com-1ecc6299db9ec823/uinput-sys-0.1.3/src/codes | sed 's/^KEY_//' | awk '{print "(\""$1"\", KEY_"$1"),"}'
        ("RESERVED", KEY_RESERVED),
        ("ESC", KEY_ESC),
        ("1", KEY_1),
        ("2", KEY_2),
        ("3", KEY_3),
        ("4", KEY_4),
        ("5", KEY_5),
        ("6", KEY_6),
        ("7", KEY_7),
        ("8", KEY_8),
        ("9", KEY_9),
        ("10", KEY_10),
        ("MINUS", KEY_MINUS),
        ("EQUAL", KEY_EQUAL),
        ("BACKSPACE", KEY_BACKSPACE),
        ("TAB", KEY_TAB),
        ("Q", KEY_Q),
        ("W", KEY_W),
        ("E", KEY_E),
        ("R", KEY_R),
        ("T", KEY_T),
        ("Y", KEY_Y),
        ("U", KEY_U),
        ("I", KEY_I),
        ("O", KEY_O),
        ("P", KEY_P),
        ("LEFTBRACE", KEY_LEFTBRACE),
        ("RIGHTBRACE", KEY_RIGHTBRACE),
        ("ENTER", KEY_ENTER),
        ("LEFTCTRL", KEY_LEFTCTRL),
        ("A", KEY_A),
        ("S", KEY_S),
        ("D", KEY_D),
        ("F", KEY_F),
        ("G", KEY_G),
        ("H", KEY_H),
        ("J", KEY_J),
        ("K", KEY_K),
        ("L", KEY_L),
        ("SEMICOLON", KEY_SEMICOLON),
        ("APOSTROPHE", KEY_APOSTROPHE),
        ("GRAVE", KEY_GRAVE),
        ("LEFTSHIFT", KEY_LEFTSHIFT),
        ("BACKSLASH", KEY_BACKSLASH),
        ("Z", KEY_Z),
        ("X", KEY_X),
        ("C", KEY_C),
        ("V", KEY_V),
        ("B", KEY_B),
        ("N", KEY_N),
        ("M", KEY_M),
        ("COMMA", KEY_COMMA),
        ("DOT", KEY_DOT),
        ("SLASH", KEY_SLASH),
        ("RIGHTSHIFT", KEY_RIGHTSHIFT),
        ("KPASTERISK", KEY_KPASTERISK),
        ("LEFTALT", KEY_LEFTALT),
        ("SPACE", KEY_SPACE),
        ("CAPSLOCK", KEY_CAPSLOCK),
        ("F1", KEY_F1),
        ("F2", KEY_F2),
        ("F3", KEY_F3),
        ("F4", KEY_F4),
        ("F5", KEY_F5),
        ("F6", KEY_F6),
        ("F7", KEY_F7),
        ("F8", KEY_F8),
        ("F9", KEY_F9),
        ("F10", KEY_F10),
        ("NUMLOCK", KEY_NUMLOCK),
        ("SCROLLLOCK", KEY_SCROLLLOCK),
        ("KP7", KEY_KP7),
        ("KP8", KEY_KP8),
        ("KP9", KEY_KP9),
        ("KPMINUS", KEY_KPMINUS),
        ("KP4", KEY_KP4),
        ("KP5", KEY_KP5),
        ("KP6", KEY_KP6),
        ("KPPLUS", KEY_KPPLUS),
        ("KP1", KEY_KP1),
        ("KP2", KEY_KP2),
        ("KP3", KEY_KP3),
        ("KP0", KEY_KP0),
        ("KPDOT", KEY_KPDOT),
        ("ZENKAKUHANKAKU", KEY_ZENKAKUHANKAKU),
        ("102ND", KEY_102ND),
        ("F11", KEY_F11),
        ("F12", KEY_F12),
        ("RO", KEY_RO),
        ("KATAKANA", KEY_KATAKANA),
        ("HIRAGANA", KEY_HIRAGANA),
        ("HENKAN", KEY_HENKAN),
        ("KATAKANAHIRAGANA", KEY_KATAKANAHIRAGANA),
        ("MUHENKAN", KEY_MUHENKAN),
        ("KPJPCOMMA", KEY_KPJPCOMMA),
        ("KPENTER", KEY_KPENTER),
        ("RIGHTCTRL", KEY_RIGHTCTRL),
        ("KPSLASH", KEY_KPSLASH),
        ("SYSRQ", KEY_SYSRQ),
        ("RIGHTALT", KEY_RIGHTALT),
        ("LINEFEED", KEY_LINEFEED),
        ("HOME", KEY_HOME),
        ("UP", KEY_UP),
        ("PAGEUP", KEY_PAGEUP),
        ("LEFT", KEY_LEFT),
        ("RIGHT", KEY_RIGHT),
        ("END", KEY_END),
        ("DOWN", KEY_DOWN),
        ("PAGEDOWN", KEY_PAGEDOWN),
        ("INSERT", KEY_INSERT),
        ("DELETE", KEY_DELETE),
        ("MACRO", KEY_MACRO),
        ("MUTE", KEY_MUTE),
        ("VOLUMEDOWN", KEY_VOLUMEDOWN),
        ("VOLUMEUP", KEY_VOLUMEUP),
        ("POWER", KEY_POWER),
        ("KPEQUAL", KEY_KPEQUAL),
        ("KPPLUSMINUS", KEY_KPPLUSMINUS),
        ("PAUSE", KEY_PAUSE),
        ("SCALE", KEY_SCALE),
        ("PSCR", KEY_SYSRQ),
        ("SLCK", KEY_SCROLLLOCK),
        ("BRK", KEY_PAUSE),
        ("GRV", KEY_GRAVE),
        ("0", KEY_10), // dumb or named wrong?
        ("MINS", KEY_MINUS),
        ("EQL", KEY_EQUAL),
        ("BSPC", KEY_BACKSPACE),
        ("LBRC", KEY_LEFTBRACE),
        ("RBRC", KEY_RIGHTBRACE),
        ("BSLS", KEY_BACKSLASH),
        ("SCLN", KEY_SEMICOLON),
        ("QUOT", KEY_APOSTROPHE),
        ("ENT", KEY_ENTER),
        ("COMM", KEY_COMMA),
        ("DOT", KEY_DOT),
        ("SLSH", KEY_SLASH),
        ("CAPS", KEY_CAPSLOCK),
        ("LSFT", KEY_LEFTSHIFT),
        ("RSFT", KEY_RIGHTSHIFT),
        ("SPC", KEY_SPACE),
        ("LCTL", KEY_LEFTCTRL),
        ("RCTL", KEY_RIGHTCTRL),
        ("LALT", KEY_LEFTALT),
        ("RALT", KEY_RIGHTALT),
        ("INS", KEY_INSERT),
        ("PGUP", KEY_PAGEUP),
        ("PGDN", KEY_PAGEDOWN),
        ("DEL", KEY_DELETE),
        ("RGHT", KEY_RIGHT),
        ("NLCK", KEY_NUMLOCK),
        ("PSLS", KEY_KPSLASH),
        ("PAST", KEY_KPASTERISK),
        ("PMNS", KEY_KPMINUS),
        ("P7", KEY_KP7),
        ("P8", KEY_KP8),
        ("P9", KEY_KP9),
        ("P4", KEY_KP4),
        ("P5", KEY_KP5),
        ("P6", KEY_KP6),
        ("PPLS", KEY_KPPLUS),
        ("P1", KEY_KP1),
        ("P2", KEY_KP2),
        ("P3", KEY_KP3),
        ("P0", KEY_KP0),
        ("PDOT", KEY_KPDOT),
        ("PENT", KEY_KPENTER),
    ]
    .iter()
    .cloned()
    .map(|(m, v)| (v as u16, m))
    .collect()
}

pub const KEY_RESERVED: c_int = 0;
pub const KEY_ESC: c_int = 1;
pub const KEY_1: c_int = 2;
pub const KEY_2: c_int = 3;
pub const KEY_3: c_int = 4;
pub const KEY_4: c_int = 5;
pub const KEY_5: c_int = 6;
pub const KEY_6: c_int = 7;
pub const KEY_7: c_int = 8;
pub const KEY_8: c_int = 9;
pub const KEY_9: c_int = 10;
pub const KEY_10: c_int = 11;
pub const KEY_MINUS: c_int = 12;
pub const KEY_EQUAL: c_int = 13;
pub const KEY_BACKSPACE: c_int = 14;
pub const KEY_TAB: c_int = 15;
pub const KEY_Q: c_int = 16;
pub const KEY_W: c_int = 17;
pub const KEY_E: c_int = 18;
pub const KEY_R: c_int = 19;
pub const KEY_T: c_int = 20;
pub const KEY_Y: c_int = 21;
pub const KEY_U: c_int = 22;
pub const KEY_I: c_int = 23;
pub const KEY_O: c_int = 24;
pub const KEY_P: c_int = 25;
pub const KEY_LEFTBRACE: c_int = 26;
pub const KEY_RIGHTBRACE: c_int = 27;
pub const KEY_ENTER: c_int = 28;
pub const KEY_LEFTCTRL: c_int = 29;
pub const KEY_A: c_int = 30;
pub const KEY_S: c_int = 31;
pub const KEY_D: c_int = 32;
pub const KEY_F: c_int = 33;
pub const KEY_G: c_int = 34;
pub const KEY_H: c_int = 35;
pub const KEY_J: c_int = 36;
pub const KEY_K: c_int = 37;
pub const KEY_L: c_int = 38;
pub const KEY_SEMICOLON: c_int = 39;
pub const KEY_APOSTROPHE: c_int = 40;
pub const KEY_GRAVE: c_int = 41;
pub const KEY_LEFTSHIFT: c_int = 42;
pub const KEY_BACKSLASH: c_int = 43;
pub const KEY_Z: c_int = 44;
pub const KEY_X: c_int = 45;
pub const KEY_C: c_int = 46;
pub const KEY_V: c_int = 47;
pub const KEY_B: c_int = 48;
pub const KEY_N: c_int = 49;
pub const KEY_M: c_int = 50;
pub const KEY_COMMA: c_int = 51;
pub const KEY_DOT: c_int = 52;
pub const KEY_SLASH: c_int = 53;
pub const KEY_RIGHTSHIFT: c_int = 54;
pub const KEY_KPASTERISK: c_int = 55;
pub const KEY_LEFTALT: c_int = 56;
pub const KEY_SPACE: c_int = 57;
pub const KEY_CAPSLOCK: c_int = 58;
pub const KEY_F1: c_int = 59;
pub const KEY_F2: c_int = 60;
pub const KEY_F3: c_int = 61;
pub const KEY_F4: c_int = 62;
pub const KEY_F5: c_int = 63;
pub const KEY_F6: c_int = 64;
pub const KEY_F7: c_int = 65;
pub const KEY_F8: c_int = 66;
pub const KEY_F9: c_int = 67;
pub const KEY_F10: c_int = 68;
pub const KEY_NUMLOCK: c_int = 69;
pub const KEY_SCROLLLOCK: c_int = 70;
pub const KEY_KP7: c_int = 71;
pub const KEY_KP8: c_int = 72;
pub const KEY_KP9: c_int = 73;
pub const KEY_KPMINUS: c_int = 74;
pub const KEY_KP4: c_int = 75;
pub const KEY_KP5: c_int = 76;
pub const KEY_KP6: c_int = 77;
pub const KEY_KPPLUS: c_int = 78;
pub const KEY_KP1: c_int = 79;
pub const KEY_KP2: c_int = 80;
pub const KEY_KP3: c_int = 81;
pub const KEY_KP0: c_int = 82;
pub const KEY_KPDOT: c_int = 83;

pub const KEY_ZENKAKUHANKAKU: c_int = 85;
pub const KEY_102ND: c_int = 86;
pub const KEY_F11: c_int = 87;
pub const KEY_F12: c_int = 88;
pub const KEY_RO: c_int = 89;
pub const KEY_KATAKANA: c_int = 90;
pub const KEY_HIRAGANA: c_int = 91;
pub const KEY_HENKAN: c_int = 92;
pub const KEY_KATAKANAHIRAGANA: c_int = 93;
pub const KEY_MUHENKAN: c_int = 94;
pub const KEY_KPJPCOMMA: c_int = 95;
pub const KEY_KPENTER: c_int = 96;
pub const KEY_RIGHTCTRL: c_int = 97;
pub const KEY_KPSLASH: c_int = 98;
pub const KEY_SYSRQ: c_int = 99;
pub const KEY_RIGHTALT: c_int = 100;
pub const KEY_LINEFEED: c_int = 101;
pub const KEY_HOME: c_int = 102;
pub const KEY_UP: c_int = 103;
pub const KEY_PAGEUP: c_int = 104;
pub const KEY_LEFT: c_int = 105;
pub const KEY_RIGHT: c_int = 106;
pub const KEY_END: c_int = 107;
pub const KEY_DOWN: c_int = 108;
pub const KEY_PAGEDOWN: c_int = 109;
pub const KEY_INSERT: c_int = 110;
pub const KEY_DELETE: c_int = 111;
pub const KEY_MACRO: c_int = 112;
pub const KEY_MUTE: c_int = 113;
pub const KEY_VOLUMEDOWN: c_int = 114;
pub const KEY_VOLUMEUP: c_int = 115;
pub const KEY_POWER: c_int = 116; /* SC System Power Down */
pub const KEY_KPEQUAL: c_int = 117;
pub const KEY_KPPLUSMINUS: c_int = 118;
pub const KEY_PAUSE: c_int = 119;
pub const KEY_SCALE: c_int = 120; /* AL Compiz Scale : c_int = Expose */
