use crate::{
    dimensions::GridIndex,
    maps::{
        MapCell,
        MapConfig,
        MapFile,
        state::MapBody,
    },
    physics::PhysicsSimulation,
    render::{
        hex_renderer::{
            HexData,
            HexRenderer,
        },
        NamedTexture,
        PointLight,
        SpriteData,
        SpriteRenderer,
    }
};
use hashbrown::{
    HashMap,
    HashSet,
};
use nalgebra::Point2;

pub struct MapState {
    cells: HashMap<GridIndex, MapCell>,
    spawns: HashSet<GridIndex>,
    light_positions: Vec<(f32, f32)>,
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

    pub fn queue_draw(&self, config: &MapConfig, hex_renderer: &mut HexRenderer, sprite_renderer: &mut SpriteRenderer, lights: &mut Vec<PointLight>) {
        let mut data = Vec::with_capacity(self.cells.len());
        for (grid_index, map_cell) in self.cells.iter() {
            data.push(HexData {
                position: *grid_index,
                height: map_cell.height,
                elevation: map_cell.elevation,
                rgba_color: map_cell.rgba_color
            });
        }
        hex_renderer.queue(self.hex_cell_length, &data);

        let mut sprite_data = Vec::with_capacity(self.light_positions.len());
        for position in self.light_positions.iter() {

            lights.push(PointLight {
                position: glm::vec3(position.0, config.light_center_height, -position.1),
                color: glm::vec3(config.light_color.0, config.light_color.1, config.light_color.2),
                attenuation: glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2),
            });

            sprite_data.push(SpriteData {
                world_bottom_center_position: glm::vec3(position.0, config.light_center_height - config.light_half_size.1, -position.1),
                world_half_size: glm::vec2(config.light_half_size.0, config.light_half_size.1),
                tex_bottom_left: glm::vec2(config.light_texel_bottom_left.0, config.light_texel_bottom_left.1),
                tex_top_right: glm::vec2(config.light_texel_top_right.0, config.light_texel_top_right.1),
            })
        }
        sprite_renderer.queue(NamedTexture::SpriteSheet1, sprite_data.as_slice());
    }
}
