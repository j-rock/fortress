use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    render::CameraConfig
};
use glm;
use nalgebra::{
    Point2,
    Vector2,
    Vector3,
};

pub struct Camera {
    config_manager: SimpleConfigManager<CameraConfig>,
    world_position: Vector3<f64>,
    curr_half_lengths: Vector2<f64>,
}

impl Camera {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Camera> {
        let config_manager: SimpleConfigManager<CameraConfig> = SimpleConfigManager::from_config_resource(config_watcher, "camera.conf")?;
        let (world_position, curr_half_lengths) = {
            let config =  config_manager.get();
            let world_position = Vector3::new(
                config.initial_position_when_no_players.0,
                config.initial_position_when_no_players.1,
                config.initial_position_when_no_players.2);
            let curr_half_lengths = Vector2::new(config.physical_min_half_lengths.0, config.physical_min_half_lengths.1);
            (world_position, curr_half_lengths)
        };
        Ok(Camera {
            config_manager,
            world_position,
            curr_half_lengths,
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

    pub fn post_update(&mut self, player_locs: Vec<Point2<f64>>) {
        if player_locs.is_empty() {
            return;
        }

        let config = self.config_manager.get();
        let mut min_coords = Vector2::new(std::f64::MAX, std::f64::MAX);
        let mut max_coords = Vector2::new(std::f64::MIN, std::f64::MIN);
        for player_loc in player_locs.into_iter() {
            min_coords.x = if min_coords.x < player_loc.coords.x { min_coords.x } else { player_loc.coords.x };
            min_coords.y = if min_coords.y < player_loc.coords.y { min_coords.y } else { player_loc.coords.y };
            max_coords.x = if max_coords.x > player_loc.coords.x { max_coords.x } else { player_loc.coords.x };
            max_coords.y = if max_coords.y > player_loc.coords.y { max_coords.y } else { player_loc.coords.y };
        }

        let mut teleported = false;
        if min_coords.x < self.world_position.x - self.curr_half_lengths.x {
            self.world_position.x = min_coords.x;
            teleported = true;
        }
        if max_coords.x > self.world_position.x + self.curr_half_lengths.x {
            self.world_position.x = max_coords.x;
            teleported = true;
        }
        if min_coords.y < self.world_position.z - self.curr_half_lengths.y {
            self.world_position.z = min_coords.y;
            teleported = true;
        }
        if max_coords.y > self.world_position.z + self.curr_half_lengths.y {
            self.world_position.z = max_coords.y;
            teleported = true;
        }
        if teleported {
            return;
        }

        let x_no_move_half_length = config.physical_no_move_ratios.0 * self.curr_half_lengths.x;
        let x_min_move = min_coords.x < self.world_position.x - x_no_move_half_length;
        let x_max_move = max_coords.x > self.world_position.x + x_no_move_half_length;
        if x_min_move && x_max_move {
            println!("Expand x");
        } else if x_min_move {
            let small_window_x = self.world_position.x - x_no_move_half_length;
            let large_window_x = self.world_position.x - self.curr_half_lengths.x;
            self.world_position.x -= (small_window_x - min_coords.x) / (min_coords.x - large_window_x) * config.physical_max_move_speed;
        } else if x_max_move {
            let small_window_x = self.world_position.x + x_no_move_half_length;
            let large_window_x = self.world_position.x + self.curr_half_lengths.x;
            self.world_position.x += (max_coords.x - small_window_x) / (large_window_x - max_coords.x) * config.physical_max_move_speed;
        }

        let z_no_move_half_length = config.physical_no_move_ratios.1 * self.curr_half_lengths.y;
        let z_min_move = min_coords.y < self.world_position.z - z_no_move_half_length;
        let z_max_move = max_coords.y > self.world_position.z + z_no_move_half_length;
        if z_min_move && z_max_move {
            println!("Expand z");
        } else if z_min_move {
            let small_window_z = self.world_position.z - z_no_move_half_length;
            let large_window_z = self.world_position.z - self.curr_half_lengths.y;
            self.world_position.z -= (small_window_z - min_coords.y) / (min_coords.y - large_window_z) * config.physical_max_move_speed;
        } else if z_max_move {
            let small_window_z = self.world_position.z + z_no_move_half_length;
            let large_window_z = self.world_position.z + self.curr_half_lengths.y;
            self.world_position.z += (max_coords.y - small_window_z) / (large_window_z - max_coords.y) * config.physical_max_move_speed;
        }
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
