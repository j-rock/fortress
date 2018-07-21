use app::StatusOr;
use file;
use notify::{
    self,
    Watcher,
};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc,
    time::Duration
};

pub struct Config {
}

pub struct ConfigWatcher {
    _watcher: notify::RecommendedWatcher,
    fs_events: mpsc::Receiver<notify::DebouncedEvent>,
    children: HashMap<String, Config>
}

impl ConfigWatcher {
    pub fn new() -> StatusOr<ConfigWatcher> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_secs(5))
            .map_err(|e| format!("Error creating watcher: {}", e))?;
        watcher.watch(file::util::resource_base(), notify::RecursiveMode::Recursive)
            .map_err(|e| format!("Error watching resource dir: {}", e))?;

        Ok(ConfigWatcher {
            _watcher: watcher,
            fs_events: rx,
            children: HashMap::new()
        })
    }

    pub fn update(&self) {
        for event in self.fs_events.try_iter() {
            match event {
                notify::DebouncedEvent::NoticeWrite(path) => self.set_dirty(path),
                notify::DebouncedEvent::Write(path) => self.set_dirty(path),
                _ => {}
            }
        }
    }

    pub fn set_dirty(&self, p: PathBuf) {
        let s = p.into_os_string().into_string();
    }
}