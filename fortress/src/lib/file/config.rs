use app::StatusOr;
use file;
use notify::{
    self,
    Watcher,
};
use serde::de::DeserializeOwned;
use serde_json;
use std::{
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData,
    path::PathBuf,
    sync::mpsc,
    time::Duration,
    rc::Rc,
};

struct DirtyBit {
    is_dirty: Rc<RefCell<bool>>
}

impl DirtyBit {
    pub fn new() -> DirtyBit {
        DirtyBit {
            is_dirty: Rc::new(RefCell::new(true))
        }
    }

    pub fn set_dirty(&self) {
        self.is_dirty.replace(true);
    }

    pub fn set_clean(&self) { self.is_dirty.replace(false); }

    pub fn is_dirty(&self) -> bool { *self.is_dirty.borrow()}
}

pub struct Config<T> {
    dirty: DirtyBit,
    path: PathBuf,
    _phantom: PhantomData<T>,
}

impl<T> Config<T> {
    fn new(dirty: DirtyBit, path: PathBuf) -> Config<T> {
        Config {
            dirty,
            path,
            _phantom: PhantomData
        }
    }
}

impl<T: DeserializeOwned> Config<T> {
    pub fn try_load(&mut self) -> StatusOr<Option<T>> {
        if self.dirty.is_dirty() {
            let reader = file::util::reader(&self.path)?;
            let parsed = serde_json::from_reader(reader)
                .map_err(|e| format!("Couldn't parse config {:?}: {}", self.path, e))?;
            self.dirty.set_clean();
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }
}

pub struct ConfigWatcher {
    _watcher: notify::RecommendedWatcher,
    fs_events: mpsc::Receiver<notify::DebouncedEvent>,
    children: HashMap<PathBuf, DirtyBit>
}

impl ConfigWatcher {
    pub fn new() -> StatusOr<ConfigWatcher> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_millis(30))
            .map_err(|e| format!("Error creating watcher: {}", e))?;
        watcher.watch(file::util::resource_base(), notify::RecursiveMode::Recursive)
            .map_err(|e| format!("Error watching resource dir: {}", e))?;

        Ok(ConfigWatcher {
            _watcher: watcher,
            fs_events: rx,
            children: HashMap::new()
        })
    }

    pub fn watch<T>(&mut self, path: PathBuf) -> StatusOr<Config<T>> {
        if !path.exists() {
            return Err(String::from(format!("Cannot watch path because it doesn't exist: {:?}", path)));
        }

        let dirty_bit = DirtyBit::new();
        let dirty_bit_copy = DirtyBit { is_dirty: Rc::clone(&dirty_bit.is_dirty) };
        self.children.insert(path.clone(), dirty_bit);
        Ok(Config::new(dirty_bit_copy, path))
    }

    pub fn update(&self) {
        for event in self.fs_events.try_iter() {
            match event {
                notify::DebouncedEvent::Write(path) => self.set_dirty(path),
                _ => {}
            }
        }
    }

    fn set_dirty(&self, p: PathBuf) {
        if let Some(dirty_bit) = self.children.get(&p) {
            (*dirty_bit).set_dirty();
        }
    }
}