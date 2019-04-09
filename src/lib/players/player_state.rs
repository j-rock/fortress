use crate::{
    audio::AudioPlayer,
    buffs::Buff,
    dimensions::{
        LrDirection,
        time::DeltaTime
    },
    physics::PhysicsSimulation,
    players::{
        Player,
        PlayerStats,
        PlayerId,
        PlayerConfig,
        state::PlayerBody
    },
    weapon::{
        Crossbow,
        Sword
    }
};

pub struct PlayerState {
    pub player_id: PlayerId,
    pub config: PlayerConfig,
    pub spawn: Vec2,
    pub stats: PlayerStats,
    pub body: PlayerBody,
    pub sword: Sword,
    pub crossbow: Crossbow,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: PlayerConfig, spawn: Vec2, physics_sim: &mut PhysicsSimulation) -> PlayerState {
        let body = PlayerBody::new(&config, spawn, physics_sim);
        let sword = Sword::new(&config);
        let crossbow = Crossbow::new(&config, physics_sim);
        let stats = PlayerStats::new(&config);
        PlayerState {
            player_id,
            config,
            spawn,
            stats,
            body,
            sword,
            crossbow,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        self.body.register(player);
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.sword.pre_update(&mut self.body, dt);
        self.crossbow.pre_update(dt);
    }

    pub fn absorb_buff(&mut self, buff: Buff) {
        match buff {
            Buff::MoveSpeed => {
                self.stats.move_speed_level += 1;
            },
            Buff::NumJumps => {
                self.stats.num_jumps_level += 1;
            },
            Buff::JumpStrength => {
                self.stats.jump_strength_level += 1;
            },

            Buff::SwordAttack => {
                self.sword.stats.damage_level += 1;
            },
            Buff::SwordKnockback => {
                self.sword.stats.knockback_strength_level += 1;
            },
            Buff::SwordSlashSpeed => {
                self.sword.stats.period_level += 1;
            },

            Buff::CrossbowAttack => {
                self.crossbow.stats.arrow_damage_level += 1;
            },
            Buff::CrossbowFiringSpeed => {
                self.crossbow.stats.firing_period_level += 1;
            },
            Buff::ArrowSpeed => {
                self.crossbow.stats.arrow_speed_level += 1;
            },
            Buff::ArrowKnockback => {
                self.crossbow.stats.arrow_knockback_strength_level += 1;
            }
        }
    }

    pub fn try_slash(&mut self, audio: &AudioPlayer) {
        self.sword.try_slash(&mut self.body, audio);
    }

    pub fn try_fire(&mut self, audio: &AudioPlayer) {
        let curr_pos = self.get_body_position();
        let curr_dir = self.get_facing_dir();
        let offset = self.config.crossbow_body_offset;
        let start_position = match curr_dir {
            LrDirection::Left => Vec2::new(curr_pos.x - offset.0, curr_pos.y + offset.1),
            LrDirection::Right => Vec2::new(curr_pos.x + offset.0, curr_pos.y + offset.1),
        };

        self.crossbow.try_fire(audio, start_position, curr_dir);
    }

    pub fn respawn(&mut self, spawn: Vec2) {
        self.spawn = spawn;
        self.body.body.data_setter.set_transform(&spawn, 0.0);
    }

    pub fn get_facing_dir(&self) -> LrDirection {
        self.body.facing_dir
    }

    pub fn get_body_position(&self) -> Vec2 {
        *self.body.body.data_setter.get_position()
    }

    pub fn get_sword_position(&self) -> Vec2 {
        let body_pos = self.get_body_position();
        let sword_offset = self.body.sword_offset_from_body;
        Vec2 {
            x: body_pos.x + sword_offset.x,
            y: body_pos.y + sword_offset.y
        }
    }
}