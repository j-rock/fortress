use buff::{
    Buff,
    BuffConfig,
    BuffBoxPlacement,
};
use physics::PhysicsSimulation;
use render::{
    BoxData,
    BoxRenderer
};

pub struct BuffBox {
    buff: Buff,
    body: BuffBody,
    is_consumed: bool,
}

impl BuffBox {
    pub fn new(config: &BuffConfig, placement: &BuffBoxPlacement, physics_sim: &mut PhysicsSimulation) -> BuffBox {
        let buff_body = BuffBody::new(config, placement, physics_sim);
        BuffBox {
            buff: placement.buff,
            body,
            is_consumed: false
        }
    }

    pub fn register(&mut self) {
        let buff_box: *const BuffBox = self as *const BuffBox;
        self.body.register(buff_box);
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        self.draw_buff_box(box_renderer);

        if self.body.buff_fixture.is_some() {
            self.draw_buff(box_renderer);
        }
    }

    fn draw_buff_box(&self, box_renderer: &mut BoxRenderer) {
        let position = {
            let position = self.body.data_setter.get_position();
            glm::vec2(position.x, position.y)
        };
        let half_size = {
            let half_size = self.body.half_size;
            glm::vec2(half_size.0, half_size.1)
        };

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

    fn draw_buff(&self, box_renderer: &mut BoxRenderer) {
        let position = {
            let position = self.body.data_setter.get_position();
            glm::vec2(position.x, position.y)
        };
        let half_size = {
            let half_size = self.body.half_size;
            glm::vec2(half_size.0, half_size.1)
        };

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
}