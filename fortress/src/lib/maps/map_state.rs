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
    math::EasingFn,
    physics::PhysicsSimulation,
    render::{
        CameraStreamBounds,
        CameraStreamInfo,
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        HexData,
        HexRenderer,
        NamedSpriteSheet,
        PointLight,
        PointLights,
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

    pub fn populate_lights(&self, config: &MapConfig, lights: &mut PointLights) {
        let queue_data = self.lights
            .iter()
            .map(|position| {
                let position = glm::vec3(position.x, config.light_center_height, -position.y);
                let color = glm::vec3(config.light_color.0, config.light_color.1, config.light_color.2);
                let attenuation = glm::vec3(config.light_attenuation.0, config.light_attenuation.1, config.light_attenuation.2);
                PointLight::new(position, color, attenuation)
            });
        lights.append(queue_data);
    }

    pub fn queue_draw(&self, config: &MapConfig, camera_stream_info: &CameraStreamInfo, hex_renderer: &mut HexRenderer, sprite_renderer: &mut FullyIlluminatedSpriteRenderer) {
        let data = self.terrain.iter().filter_map(|grid_index| {
            match camera_stream_info.compute_grid_bounds(*grid_index) {
                CameraStreamBounds::Inside => {
                    Some(HexData {
                        position: *grid_index,
                        height: 1.0,
                        elevation: 0.0,
                        alpha: 1.0,
                    })
                },
                CameraStreamBounds::Margin(margin) => {
                    let elevation = config.stream_cell_min_elevation * (1.0 - EasingFn::ease_out_quad(margin));
                    let alpha = EasingFn::ease_in_cuartic(margin);

                    Some(HexData {
                        position: *grid_index,
                        height: 1.0,
                        elevation,
                        alpha,
                    })
                }
                _ => None,
            }
        });
        hex_renderer.queue(config.cell_length, data);

        let sprite_data = self.lights.iter().map(|position| {
            FullyIlluminatedSpriteData {
                world_center_position: glm::vec3(position.x, config.light_center_height, -position.y),
                world_half_size: glm::vec2(config.light_half_size.0, config.light_half_size.1),
                sprite_frame_id: SpriteSheetFrameId::new(String::from("lantern.png"), NamedSpriteSheet::SpriteSheet1),
                frame: 0,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse: Reverse::none(),
            }
        });
        sprite_renderer.queue(sprite_data);
    }
}
