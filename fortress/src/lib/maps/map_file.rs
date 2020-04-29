use crate::{
    app::StatusOr,
    dimensions::GridIndex,
    file::{
        Config,
        ConfigWatcher,
        SimpleConfigManager,
        self,
    },
    maps::MapFileConfig,
    render::Png,
};
use std::path::PathBuf;

pub struct MapFile {
    terrain: Vec<GridIndex>,
    player_spawns: Vec<GridIndex>,
    lights: Vec<GridIndex>,
    enemy_generator: Vec<GridIndex>,
    barrels: Vec<GridIndex>,
}

impl MapFile {
    fn new<'a>(config: &MapFileConfig, fragments: impl Iterator<Item=&'a MapFileFragment>) -> StatusOr<MapFile> {
        let mut terrain =  Vec::with_capacity(config.terrain_count_guess);
        let mut player_spawns = Vec::with_capacity(config.spawn_count_guess);
        let mut lights = Vec::with_capacity(config.lights_count_guess);
        let mut enemy_generator = Vec::with_capacity(config.generators_count_guess);
        let mut barrels = Vec::with_capacity(config.barrel_count_guess);

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
                        (255, 255, 0) => {
                            barrels.push(grid_index);
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
            enemy_generator,
            barrels,
        })
    }

    pub fn terrain(&self) -> &[GridIndex] {
        self.terrain.as_slice()
    }

    pub fn player_spawns(&self) -> &[GridIndex] {
        self.player_spawns.as_slice()
    }

    pub fn lights(&self) -> &[GridIndex] {
        self.lights.as_slice()
    }

    pub fn enemy_generators(&self) -> &[GridIndex] {
        self.enemy_generator.as_slice()
    }

    pub fn barrels(&self) -> &[GridIndex] {
        self.barrels.as_slice()
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
    pub fn new(config: &MapFileConfig, config_watcher: &mut ConfigWatcher) -> StatusOr<MapFileManager> {
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

        let mut fragment_managers = Vec::with_capacity(config.num_fragments);
        for path in dir_contents {
            let fragment_manager = SimpleConfigManager::<MapFileFragment>::from_resource_path(config_watcher, path)?;
            fragment_managers.push(fragment_manager);
        }

        let fragments = fragment_managers
            .iter()
            .map(|manager| manager.get());
        let map_file = MapFile::new(config, fragments)?;

        Ok(MapFileManager {
            fragment_managers,
            map_file,
        })
    }

    pub fn update(&mut self, config: &MapFileConfig) -> bool {
        let mut dirty = false;
        for manager in self.fragment_managers.iter_mut() {
            dirty |= manager.update();
        }

        if !dirty { return false; }

        let fragments = self.fragment_managers
            .iter()
            .map(|manager| manager.get());

        match MapFile::new(config, fragments) {
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
