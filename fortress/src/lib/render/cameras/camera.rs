use crate::{
    app::StatusOr,
    dimensions::time::DeltaTime,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    render::{
        CameraConfig,
        CameraStreamInfo,
    },
};
use glm;
use nalgebra::{
    Point2,
    Point3,
    Vector2,
};

pub struct Camera {
    config_manager: SimpleConfigManager<CameraConfig>,
    world_position: Point3<f64>,
}

impl Camera {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Camera> {
        let config_manager: SimpleConfigManager<CameraConfig> = SimpleConfigManager::from_config_resource(config_watcher, "camera.conf")?;
        let world_position = {
            let config =  config_manager.get();
            let world_position = Point3::new(
                config.initial_position_when_no_players.0,
                config.initial_position_when_no_players.1,
                config.initial_position_when_no_players.2);
            world_position
        };
        Ok(Camera {
            config_manager,
            world_position,
        })
    }

    pub fn projection(&self, screen_size: glm::IVec2) -> glm::Mat4 {
        let config = self.config_manager.get();
        let right = 1.0 / (2.0 * config.zoom);
        let left = -right;
        let top = (screen_size.y as f32) / (2.0 * config.zoom * screen_size.x as f32);
        let bottom = -top;
        Self::ortho(left, right, bottom, top, config.z_near, config.z_far)
    }

    pub fn view(&self, lookat: glm::Vec3, up: glm::Vec3) -> glm::Mat4 {
        let position = self.position();
        glm::ext::look_at(position, position + lookat, up)
    }

    pub fn position_independent_view(&self, lookat: glm::Vec3, up: glm::Vec3) -> glm::Mat4 {
        glm::ext::look_at(glm::vec3(0.0, 0.0, 0.0), lookat, up)
    }

    pub fn lookat_right_and_up(&self) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
        let config = self.config_manager.get();
        let lookat = glm::builtin::normalize(glm::vec3(config.lookat.0, config.lookat.1, config.lookat.2));
        let right = glm::builtin::normalize(glm::vec3(config.right.0, config.right.1, config.right.2));
        let up = glm::builtin::cross(right, lookat);
        (lookat, right, up)
    }

    pub fn position(&self) -> glm::Vec3 {
        let config = self.config_manager.get();
        glm::vec3(self.world_position.x as f32 + config.camera_pos_offset.0,
                  self.world_position.y as f32,
                  -self.world_position.z as f32 + config.camera_pos_offset.1)
    }

    pub fn pre_update(&mut self) {
        self.config_manager.update();
    }

    pub fn post_update(&mut self, player_locs: Vec<Point2<f64>>, dt: DeltaTime) {
        if player_locs.is_empty() {
            return;
        }
        if player_locs.len() > 1 {
            panic!("Can't support multiplayer");
        }

        let config = self.config_manager.get();
        let player_pos = player_locs[0];
        let player_world_pos = Point3::new(player_pos.x, 0.0, player_pos.y);

        let mut player_camera_displacement = player_world_pos - self.world_position;
        player_camera_displacement.y = 0.0;

        if player_camera_displacement.x.abs() < config.physical_no_move_half_lengths.0 {
            player_camera_displacement.x = 0.0;
        }
        if player_camera_displacement.z.abs() < config.physical_no_move_half_lengths.1 {
            player_camera_displacement.z = 0.0;
        }

        let move_multiplier = dt.as_f64_seconds() / config.physical_follow_player_factor;
        self.world_position += move_multiplier * player_camera_displacement;
    }

    pub fn stream_info(&self, hex_cell_length: f64) -> CameraStreamInfo {
        let config = self.config_manager.get();
        let cam_pos = Point2::new(self.world_position.x, -self.world_position.z);
        let inside_half_extents = Vector2::new(config.stream_inside_half_extents.0, config.stream_inside_half_extents.1);
        CameraStreamInfo::new(cam_pos, inside_half_extents, config.stream_margin_length, hex_cell_length)
    }

    fn ortho(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> glm::Mat4 {
        let rml = right - left;
        let tmb = top - bottom;
        let fmn = z_far - z_near;

        glm::Matrix4::new(
            glm::vec4(2.0 / rml, 0.0, 0.0, 0.0),
            glm::vec4(0.0, 2.0 / tmb, 0.0, 0.0),
            glm::vec4(0.0, 0.0, -2.0 / fmn, 0.0),
            glm::vec4(-(right + left) / rml, -(top + bottom) / tmb, -(z_far + z_near) / fmn, 1.0))
    }
}
