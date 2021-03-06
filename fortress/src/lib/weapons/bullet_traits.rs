use crate::players::PlayerBulletConfig;
use glm;

pub struct BulletTraits {
    attack_type: BulletAttackType,
    element: BulletElement,
}

impl BulletTraits {
    pub fn new(attack_type: BulletAttackType, element: BulletElement) -> Self {
        BulletTraits {
            attack_type,
            element
        }
    }

    pub fn remove_on_collision(&self) -> bool {
        match self.attack_type {
            BulletAttackType::Regular => true,
            BulletAttackType::Special => false,
        }
    }

    pub fn sprite_sheet_image_name(&self) -> &'static str {
        match self.element {
            BulletElement::Fire => "shooting_fireball.png",
            BulletElement::Poison => "shooting_poisonball.png",
            BulletElement::Ice => "shooting_iceball.png",
        }
    }

    pub fn light_color(&self, config: &PlayerBulletConfig) -> glm::Vec3 {
        self.element.light_color(config)
    }
}

#[derive(Copy, Clone)]
pub enum BulletAttackType {
    Regular,
    Special
}

#[derive(Copy, Clone)]
pub enum BulletElement {
    Fire,
    Poison,
    Ice,
}

impl BulletElement {
    pub fn light_color(self, config: &PlayerBulletConfig) -> glm::Vec3 {
        match self {
            Self::Fire => {
                glm::vec3(config.light_color_fire.0,
                          config.light_color_fire.1,
                          config.light_color_fire.2)
            },
            Self::Poison => {
                glm::vec3(config.light_color_poison.0,
                          config.light_color_poison.1,
                          config.light_color_poison.2)
            },
            Self::Ice => {
                glm::vec3(config.light_color_ice.0,
                          config.light_color_ice.1,
                          config.light_color_ice.2)
            },
        }
    }
}

