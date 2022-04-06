use std::{path::{PathBuf, Path}, sync::mpsc::{Sender, Receiver}, fs::File, time::Duration};

use notify::{DebouncedEvent, RecommendedWatcher, Watcher, RecursiveMode};
use anyhow::Result;


pub struct WatchJob {
    path: PathBuf,
    watcher: RecommendedWatcher,
    request_filename: String,
    completed_filename: String,
    rx: Receiver<DebouncedEvent>,
}

impl WatchJob{
    pub fn new<P: AsRef<Path>>(watch_path: P) -> Result<Self> {
        let path = watch_path.as_ref();
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500))?;

        let mut instance = WatchJob{
            path: path.to_path_buf(),
            watcher,
            request_filename: "request.marker".into(),
            completed_filename: "completed.marker".into(),
            rx
        };

        instance.clean()?;

        Ok(instance)
    }

    pub fn clean(&mut self) -> Result<()>{
        if let Err(e) = self.watcher.unwatch(&self.path) {
            println!("Failed to unwatch: {:#?}", e);
        }

        for entry in std::fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
    
            if entry.file_type()?.is_dir() {
                std::fs::remove_dir_all(path)?;
            } else {
                std::fs::remove_file(path)?;
            }
        }

        self.watcher.watch(&self.path, RecursiveMode::NonRecursive)?;

        Ok(())
    }

    pub fn request_and_block(&self) -> Result<()>{
        drop(File::create(
            self.path.join(&self.request_filename)
        )?);

        loop {
            match self.rx.recv() {
                Ok(DebouncedEvent::Create(path)) 
                if path.file_name().map(|f| f.to_string_lossy() == self.completed_filename).unwrap_or_default() => {
                    println!("Completed: {:?}", path);
                    break;
                },
                
                Ok(event) => println!("Ignore {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
        
        Ok(())
    }
}