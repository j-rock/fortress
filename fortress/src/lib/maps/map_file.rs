use crate::{
    app::StatusOr,
    dimensions::GridIndex,
    file::{
        Config,
        ConfigWatcher,
        SimpleConfigManager,
        self,
    },
    render::Png,
};
use std::path::PathBuf;

pub struct MapFile {
    terrain: Vec<GridIndex>,
    player_spawns: Vec<GridIndex>,
    lights: Vec<GridIndex>,
    enemy_generator: Vec<GridIndex>,
}

impl MapFile {
    fn new<'a>(fragments: impl Iterator<Item=&'a MapFileFragment>) -> StatusOr<MapFile> {
        // TODO: use config to preallocate capacities.
        let mut terrain =  Vec::with_capacity(10000);
        let mut player_spawns = Vec::with_capacity(2);
        let mut lights = Vec::with_capacity(100);
        let mut enemy_generator = Vec::with_capacity(10);

        // TODO: in the future, there will be more than one fragment.
        let mut num_fragments = 0;
        for fragment in fragments {
            num_fragments += 1;
            // Assume terrain, spawns, lights, enemies are all in first fragment.
            let (width, height) = fragment.image.size();
            let image_bytes = fragment.image.bytes();
            for row in 0..height {
                for col in 0..width {
                    let start_idx = 4 * (row * width + col);
                    let red = image_bytes[start_idx];
                    let green = image_bytes[start_idx + 1];
                    let blue = image_bytes[start_idx + 2];

                    let grid_index = GridIndex::new(col as i64, row as i64);
                    match (red, green, blue) {
                        (0, 0, 0) => {
                            terrain.push(grid_index);
                        },
                        (0, 255, 0) => {
                            player_spawns.push(grid_index);
                            terrain.push(grid_index);
                        },
                        (0, 0, 255) => {
                            lights.push(grid_index);
                        },
                        (255, 0, 0) => {
                            enemy_generator.push(grid_index);
                            terrain.push(grid_index);
                        },
                        _ => {},
                    }
                }
            }
        }

        if num_fragments > 1 {
            return Err(format!("More than one map fragment ({})", num_fragments));
        }

        Ok(MapFile {
            terrain,
            player_spawns,
            lights,
            enemy_generator
        })
    }

    pub fn terrain(&self) -> &Vec<GridIndex> {
        &self.terrain
    }

    pub fn player_spawns(&self) -> &Vec<GridIndex> {
        &self.player_spawns
    }

    pub fn lights(&self) -> &Vec<GridIndex> {
        &self.lights
    }

    pub fn enemy_generators(&self) -> &Vec<GridIndex> {
        &self.enemy_generator
    }
}

// In theory, we construct a MapFile from multiple images.
struct MapFileFragment {
    image: Png,
}

impl Config for MapFileFragment {
    fn from_path(path_buf: &PathBuf) -> StatusOr<Self> {
        Ok(MapFileFragment {
            image: Png::from_file(path_buf)?
        })
    }
}

pub struct MapFileManager {
    fragment_managers: Vec<SimpleConfigManager<MapFileFragment>>,
    map_file: MapFile
}

impl MapFileManager {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<MapFileManager> {
        let mut map_dir = file::util::resource_base();
        map_dir.push("map");

        let dir_contents =
            map_dir
            .read_dir()
            .map_err(|e| format!("MapFileManager: {:?}", e))?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path: &PathBuf| {
                if let Some(os_base_name) = path.file_name() {
                    if let Some(base_name) = os_base_name.to_str() {
                        if base_name.ends_with(".png") {
                            return true;
                        }
                    }
                }
                false
            });

        // TODO: estimate capacity.
        let mut fragment_managers = Vec::with_capacity(10);
        for path in dir_contents {
            let fragment_manager = SimpleConfigManager::<MapFileFragment>::from_resource_path(config_watcher, path)?;
            fragment_managers.push(fragment_manager);
        }

        let fragments = fragment_managers
            .iter()
            .map(|manager| manager.get());
        let map_file = MapFile::new(fragments)?;

        Ok(MapFileManager {
            fragment_managers,
            map_file,
        })
    }

    pub fn update(&mut self) -> bool {
        let mut dirty = false;
        for manager in self.fragment_managers.iter_mut() {
            dirty |= manager.update();
        }

        if !dirty { return false; }

        let fragments = self.fragment_managers
            .iter()
            .map(|manager| manager.get());

        match MapFile::new(fragments) {
            Ok(map_file) => {
                self.map_file = map_file;
                true
            },
            _ => false
        }
    }

    pub fn get(&self) -> &MapFile {
        &self.map_file
    }
}
