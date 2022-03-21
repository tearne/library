use std::collections::HashMap;

use libc::input_event;

use crate::error::Error;

pub mod device;
pub mod monitor;

pub struct KeyBuffer {
    key_buffer: Vec<&'static str>,
    max_length: usize,
    key_map: HashMap<u16, &'static str>,
}
impl KeyBuffer {
    pub fn new(length: usize) -> Self {
        KeyBuffer {
            key_buffer: Vec::<&str>::new(),
            max_length: length,
            key_map: device::key_map(),
        }
    }

    pub fn log_event(&mut self, e_res: &Result<input_event, Error>) {
        let t = e_res.iter();
        t.for_each(|e| {
            self.key_buffer
                .push(self.key_map.get(&e.code).unwrap_or(&""));
            if self.key_buffer.len() > self.max_length {
                self.key_buffer
                    .remove(self.key_buffer.len() - self.max_length - 1);
            }
        });
    }

    pub fn contains(&self, sequence: &[&str]) -> bool {
        let buf_length = self.key_buffer.len();
        let seq_length = sequence.len();

        if buf_length < seq_length {
            false
        } else {
            let section: &[&str] = &self.key_buffer[(buf_length - seq_length)..];
            section == sequence
        }
    }
}
