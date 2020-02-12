use crate::{
    dimensions::{
        GridIndex,
        Reverse,
    },
    maps::{
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
use nalgebra::{
    Point2,
    Vector2,
};
use std::collections::HashSet;

pub struct MapState {
    terrain: HashSet<GridIndex>,
    player_spawns: Vec<Point2<f64>>,
    lights: Vec<Point2<f32>>,
    enemy_generators: Vec<Point2<f64>>,
    _body: MapBody,
}

impl MapState {
    pub fn new(config: &MapConfig, map_file: &MapFile, physics_sim: &mut PhysicsSimulation) -> MapState {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(config.cell_length);

        let terrain: HashSet<_> = map_file.terrain()
            .iter()
            .cloned()
            .collect();

        let player_spawns: Vec<_> = map_file.player_spawns()
            .iter()
            .map(|grid_index| {
                grid_index.index_center(&axial_to_cartesian)
            })
            .collect();

        let lights = map_file.lights()
            .iter()
            .map(|grid_index| {
                let p = grid_index.index_center(&axial_to_cartesian);
                Point2::from(Vector2::new(p.x as f32, p.y as f32))
            })
            .collect();

        let enemy_generators: Vec<_> = map_file.enemy_generators()
            .iter()
            .map(|grid_index| {
                grid_index.index_center(&axial_to_cartesian)
            })
            .collect();

        let body = MapBody::new(config, &terrain, physics_sim);

        MapState {
            terrain,
            player_spawns,
            lights,
            enemy_generators,
            _body: body,
        }
    }

    pub fn player_spawns(&self) -> &Vec<Point2<f64>> {
        &self.player_spawns
    }

    pub fn enemy_generators(&self) -> &Vec<Point2<f64>> {
        &self.enemy_generators
    }

    pub fn populate_lights(&self, config: &MapConfig, lights: &mut Vec<PointLight>) {
        for position in self.lights.iter() {
            lights.push(PointLight {
                position: glm::vec3(position.x, config.light_center_height, -position.y),
                color: glm::vec3(config.light_color.0, config.light_color.1, config.light_color.2),
                attenuation: glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2),
            });
        }
    }

    pub fn queue_draw(&self, config: &MapConfig, hex_renderer: &mut HexRenderer, sprite_renderer: &mut FullyIlluminatedSpriteRenderer) {
        let data = self.terrain.iter().map(|grid_index| {
            HexData {
                position: *grid_index,
                height: 1.0,
                elevation: 0.0,
            }
        });
        hex_renderer.queue(config.cell_length, data);

        let sprite_data = self.lights.iter().map(|position| {
            FullyIlluminatedSpriteData {
                world_center_position: glm::vec3(position.x, config.light_center_height, -position.y),
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
