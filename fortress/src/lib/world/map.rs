use app::StatusOr;
use file::{
    ConfigWatcher,
    ConfigLoader,
    self,
};
use liquidfun;
use world::PhysicsSimulation;

#[derive(Debug, Deserialize)]
struct Platform {
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32
}

#[derive(Deserialize)]
struct MapData {
    platforms: Vec<Platform>
}

pub struct Map {
    config_loader: ConfigLoader<MapData>,
    platform_body: liquidfun::box2d::dynamics::body::Body,
}

impl Map {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<Map> {
        let map_config = file::util::resource_path("config", "map.conf");
        let mut config_loader: ConfigLoader<MapData> = ConfigWatcher::watch(config_watcher, map_config)?;
        let map_data = config_loader.force_load()?;
        let platform_body = Self::create_body_from_platforms(map_data.platforms, physics_sim.get_world_mut());
        Ok(Map {
            config_loader,
            platform_body
        })
    }

    pub fn update(&mut self) {
        let reloaded = self.config_loader.try_load();
        match reloaded {
            Err(message) => println!("Error reloading map.conf: {}", message),
            Ok(None) => {},
            Ok(Some(map_data)) => {
                self.redeploy_platforms(map_data.platforms);
            }
        }
    }

    fn redeploy_platforms(&mut self, _platforms: Vec<Platform>) {
        let mut world = self.platform_body.get_world();
        // Invalidates self.platform_body. Must quickly reset platform_body.
        world.destroy_body(&self.platform_body);
        // self.platform_body = Self::create_body_from_platforms(platforms, &mut world);
    }

    fn create_body_from_platforms(platforms: Vec<Platform>, world: &mut liquidfun::box2d::dynamics::world::World) -> liquidfun::box2d::dynamics::body::Body {
        let mut body_def = liquidfun::box2d::dynamics::body::BodyDef::default();
        let platform_body = world.create_body(&body_def);
        let mut poly_shape = liquidfun::box2d::collision::shapes::polygon_shape::PolygonShape::new();
        for platform in platforms.iter() {
            let (hx, hy) = (platform.width as f32 / 2.0, platform.height as f32 / 2.0);
            poly_shape.set_as_box(hx, hy);
            body_def.position.x = platform.top_left_x as f32 + hx;
            body_def.position.y = platform.top_left_y as f32 - hy;
            let fixture_def = liquidfun::box2d::dynamics::fixture::FixtureDef::new(&poly_shape);
            platform_body.create_fixture(&fixture_def);
        }
        platform_body
    }
}