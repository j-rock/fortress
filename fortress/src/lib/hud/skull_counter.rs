use crate::{
    hud::{
        PlayerHudUpdate,
        SkullCounterConfig,
    },
    text::{
        NamedText,
        RasterSize,
        TextContent,
        TextRenderer,
        TextRenderRequest,
    },
};
use glm;

pub struct SkullCounter {
    num_skulls: i64
}

impl SkullCounter {
    pub fn new() -> Self {
        SkullCounter {
            num_skulls: 0
        }
    }

    pub fn post_update(&mut self, player_hud_update: &PlayerHudUpdate) {
        if let Some(player1) = player_hud_update.get_first() {
            self.num_skulls = player1.skulls_collected as i64;
        }
    }

    pub fn queue_draw(&self, config: &SkullCounterConfig, text: &mut TextRenderer) {
        let content = [TextContent::Text(NamedText::SkullCounterPrefix), TextContent::Number(self.num_skulls)];
        let screen_position_percentage = glm::vec3(config.screen_pos.0, config.screen_pos.1, config.screen_pos.2);
        let drop_position_percentage = screen_position_percentage + glm::vec3(config.drop_pos_offset.0, config.drop_pos_offset.1, config.drop_pos_offset.2);

        text.queue(
            content.iter().copied(),
            TextRenderRequest {
                screen_position_percentage: drop_position_percentage,
                raster_size: RasterSize::Large,
                color: glm::vec3(config.drop_color.0, config.drop_color.1, config.drop_color.2),
                alpha: config.alpha
            });
        text.queue(
            content.iter().copied(),
            TextRenderRequest {
                screen_position_percentage,
                raster_size: RasterSize::Large,
                color: glm::vec3(config.color.0, config.color.1, config.color.2),
                alpha: config.alpha
            });
    }
}