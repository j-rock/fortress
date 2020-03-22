pub mod hero;
pub mod player;
pub mod player_config;
pub mod player_id;
pub mod player_matchers;
pub mod player_system;
pub mod state;

pub use self::hero::Hero;
pub use self::player::Player;
pub use self::player_config::PlayerConfig;
pub use self::player_config::PlayerBulletConfig;
pub use self::player_config::PlayerItemConfig;
pub use self::player_config::PlayerSystemConfig;
pub use self::player_id::PlayerId;
pub use self::player_matchers::PlayerMatchers;
pub use self::player_system::PlayerSystem;

pub const MAX_PLAYERS: usize = 4;
