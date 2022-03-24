use std::time::Duration;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use anyhow::Result;

enum State{
    Init, Order, Pending, Complete
}
impl State {
    pub fn expected_next(&self) -> Self {
        match self {
            State::Init => State::Order,
            State::Order => State::Pending,
            State::Pending => State::Complete,
            State::Complete => State::Init,
        }
    }

    pub fn filename(&self) -> &str {
        match self {
            State::Init => "0_INIT.marker",
            State::Order => "1_ORDER.marker",
            State::Pending => "2_PENDING.marker",
            State::Complete => "3_COMPLETE.marker",
        }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500))?;
    watcher.watch("/home/test/notify", RecursiveMode::Recursive)?;

    let mut state = State::Init;
    loop {
        match rx.recv() {
            Ok(de) => match de {
                notify::DebouncedEvent::NoticeWrite(path) if path.file_name() == state.expected_next() => todo!(),
                de => println!("Unhandled event {:?}", de),
            },
            Err(_) => todo!(),
        }
    }


    // Dir needn't start empty, but mustn't contain ORDER.marker

    // Place config to represent order - ignored by worker

    // Place ORDER.marker

    // Expect ACK.marker

    // Later, results files will appear, but shold be ignored until...

    // COMPLETE.marker appears

    // Any further changes are ignored by both sides until ORDER.marker appreas

    Ok(())
}
