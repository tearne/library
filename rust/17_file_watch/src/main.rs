mod watcher;

use std::{path::Path, fs::write};
use watcher::WatchJob;

fn main() -> anyhow::Result<()> {
    let watch_dir = Path::new("/dev/shm/watch_dir");

    let mut watcher = WatchJob::new(watch_dir)?;

    // Pretend payload
    write(
        watch_dir.join("myOrder.txt"), 
        "Pepperoni with goats cheese.")
    ?;
    // Create the "order" marker and wait for the "completed signal"
    watcher.request_and_block()?;
    println!("order completed: {}", std::fs::read_to_string(watch_dir.join("results.txt")).unwrap());

    // Clean the folder and place another order
    watcher.clean()?;
    write(
        watch_dir.join("anotherOrder.txt"), 
        "Ice cream.")
    ?;
    watcher.request_and_block()?;
    println!("order completed: {}", std::fs::read_to_string(watch_dir.join("results.txt")).unwrap());

    Ok(())
}
