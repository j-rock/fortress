use audio::Sound;
use buff::{
    Buff,
    BuffBody,
    BuffBoxConfig,
    BuffBoxPlacement,
    BuffConfig,
    BuffDropConfig,
};
use entity::{
    Entity,
    EntityType
};
use glm;
use physics::{
    CollisionMatcher,
    PhysicsSimulation
};
use player::Player;
use render::{
    BoxData,
    BoxRenderer
};

pub struct BuffBox {
    body: BuffBody,
    buff: Buff,
    needs_to_drop: bool,
    has_dropped: bool,
    destroy_drop: bool,
}

impl BuffBox {
    pub fn new(config: &BuffBoxConfig, placement: &BuffBoxPlacement, physics_sim: &mut PhysicsSimulation) -> BuffBox {
        let body = BuffBody::new(config, placement, physics_sim);
        BuffBox {
            body,
            buff: placement.buff,
            needs_to_drop: false,
            has_dropped: false,
            destroy_drop: false,
        }
    }

    pub fn register(&mut self) {
        let buff_box: *const BuffBox = self as *const BuffBox;
        self.body.register(buff_box);
    }

    pub fn post_update(&mut self, config: &BuffDropConfig, physics_sim: &mut PhysicsSimulation) {
        if self.needs_to_drop {
            self.needs_to_drop = false;

            if !self.has_dropped {
                self.has_dropped = true;

                let buff_box: *const BuffBox = self as *const BuffBox;
                let entity = Entity::new(EntityType::BuffDrop, buff_box);
                self.body.launch_drop(config, entity, physics_sim);
            }
        }

        if self.destroy_drop {
            self.destroy_drop = false;
            self.body.destroy_drop();
        }
    }

    pub fn draw(&self, config: &BuffConfig, box_renderer: &mut BoxRenderer) {
        self.draw_buff_drop(config, box_renderer);
        self.draw_buff_box(config, box_renderer);
    }

    fn draw_buff_box(&self, config: &BuffConfig, box_renderer: &mut BoxRenderer) {
        let position = {
            let position = self.body.buff_box_body.data_setter.get_position();
            glm::vec2(position.x, position.y)
        };
        let half_size = glm::vec2(config.buff_box.size.0 / 2.0, config.buff_box.size.1 / 2.0);

        box_renderer.queue(&[
            BoxData {
                position,
                half_size,
                rgba_tl: glm::vec4(0.4, 0.1, 1.0, 0.0),
                rgba_tr: glm::vec4(0.3, 0.2, 1.0, 0.0),
                rgba_bl: glm::vec4(0.2, 0.3, 1.0, 0.0),
                rgba_br: glm::vec4(0.1, 0.4, 1.0, 0.0),
            }
        ]);
    }

    fn draw_buff_drop(&self, config: &BuffConfig, box_renderer: &mut BoxRenderer) {
        if let Some((x, y)) = self.body.get_drop_body_position() {
            let half_size = glm::vec2(config.buff_drop.size.0 / 2.0, config.buff_drop.size.1 / 2.0);
            box_renderer.queue(&[
                BoxData {
                    position: glm::vec2(x, y),
                    half_size,
                    rgba_tl: glm::vec4(1.0, 0.2, 0.1, 0.0),
                    rgba_tr: glm::vec4(1.0, 0.5, 0.2, 0.0),
                    rgba_bl: glm::vec4(0.2, 0.8, 0.8, 0.0),
                    rgba_br: glm::vec4(0.2, 0.8, 0.8, 0.0),
                }
            ]);
        }
    }

    pub fn player_slashed_buff_box() -> CollisionMatcher {
        CollisionMatcher::match_two(EntityType::PlayerSwordSensor, EntityType::BuffBox, Box::new(|_audio, _sword_ent, buff_box_ent| {
            let buff_box: &mut Self = buff_box_ent.resolve();
            buff_box.needs_to_drop = true;
        }))
    }

    pub fn player_hit_buff_drop() -> CollisionMatcher {
        CollisionMatcher::match_two(EntityType::Player, EntityType::BuffDrop, Box::new(|audio, player_ent, buff_box_ent| {
            let player: &mut Player = player_ent.resolve();
            let buff_box: &mut Self = buff_box_ent.resolve();
            buff_box.destroy_drop = true;
            player.absorb_buff(buff_box.buff);

            audio.play_sound(Sound::Powerup);
        }))
    }
}