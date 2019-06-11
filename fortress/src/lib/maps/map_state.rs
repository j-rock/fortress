use crate::{
    dimensions::{
        GridIndex,
        Reverse,
    },
    enemies::EnemyGeneratorSpawn,
    maps::{
        MapCell,
        MapConfig,
        MapFile,
        state::MapBody,
    },
    physics::PhysicsSimulation,
    render::{
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        HexData,
        HexRenderer,
        NamedSpriteSheet,
        PointLight,
        SpriteSheetFrameId,
    }
};
use hashbrown::{
    HashMap,
    HashSet,
};
use nalgebra::{
    Point2,
    Vector2,
};

pub struct MapState {
    cells: HashMap<GridIndex, MapCell>,
    spawns: HashSet<GridIndex>,
    light_positions: Vec<(f32, f32)>,
    enemy_generator_spawns: Vec<EnemyGeneratorSpawn>,
    _body: MapBody,
    hex_cell_length: f64,
}

impl MapState {
    pub fn new(config: &MapConfig, map_file: &MapFile, physics_sim: &mut PhysicsSimulation) -> MapState {
        let cells: HashMap<_, _> = map_file.cells
            .iter()
            .map(|map_file_cell| {
                (map_file_cell.grid_index(), MapCell::from_map_file_cell(map_file_cell))
            })
            .collect();

        let spawns: HashSet<_> = map_file.cells
            .iter()
            .filter_map(|map_file_cell| {
                if !map_file_cell.is_spawn() {
                    return None;
                }
                return Some(map_file_cell.grid_index());
            })
            .collect();

        let enemy_generator_spawns: Vec<_> = map_file.enemy_generator_spawns.clone();

        let light_positions = map_file.lights
            .iter()
            .map(|map_file_light| -> (f32, f32) {
                map_file_light.position
            })
            .collect();

        let body = MapBody::new(config, &cells, physics_sim);

        MapState {
            cells,
            spawns,
            light_positions,
            enemy_generator_spawns,
            _body: body,
            hex_cell_length: config.cell_length
        }
    }

    pub fn spawns(&self) -> Vec<Point2<f64>> {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(self.hex_cell_length);
        self.spawns
            .iter()
            .map(|grid_index| {
                grid_index.index_center(&axial_to_cartesian)
            })
            .collect()
    }

    pub fn enemy_generator_spawns(&self) -> Vec<EnemyGeneratorSpawn> {
        self.enemy_generator_spawns.clone()
    }

    pub fn populate_lights(&self, config: &MapConfig, lights: &mut Vec<PointLight>) {
        for position in self.light_positions.iter() {
            lights.push(PointLight {
                position: glm::vec3(position.0, config.light_center_height, -position.1),
                color: glm::vec3(config.light_color.0, config.light_color.1, config.light_color.2),
                attenuation: glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2),
            });
        }
    }

    pub fn queue_draw(&self, config: &MapConfig, hex_renderer: &mut HexRenderer, sprite_renderer: &mut FullyIlluminatedSpriteRenderer) {
        let data = self.cells.iter().map(|(grid_index, map_cell)| {
            HexData {
                position: *grid_index,
                height: map_cell.height,
                elevation: map_cell.elevation,
            }
        });
        hex_renderer.queue(self.hex_cell_length, data);

        let sprite_data = self.light_positions.iter().map(|position| {
            FullyIlluminatedSpriteData {
                world_center_position: glm::vec3(position.0, config.light_center_height, -position.1),
                world_half_size: glm::vec2(config.light_half_size.0, config.light_half_size.1),
                sprite_frame_id: SpriteSheetFrameId {
                    name: String::from("lantern.png"),
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame: 0,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse: Reverse::none(),
            }
        });
        sprite_renderer.queue(sprite_data);
    }
}
