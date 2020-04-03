use crate::{
    locales::{
        NamedLocale,
        LocalizedTextKey,
    },
    render::TextWarehouse,
};
use glm;
use glyph_brush::{
    rusttype::Scale,
    Section
};

pub enum TextKey {
    Number(i64),
    Text(LocalizedTextKey),
}

pub struct TextRenderOrder {
    pub key: TextKey,
    pub screen_position: glm::Vec2,
    pub color: glm::Vec4,
    pub size: f32,
}

impl TextRenderOrder {
    pub fn to_section(self, locale: NamedLocale, warehouse: &mut TextWarehouse) -> Option<Section> {
        let text = match self.key {
            TextKey::Number(number) => {
                unsafe {
                    warehouse.get_number(number)?.dereference()
                }
            },
            TextKey::Text(localized_text_key) => {
                warehouse.get_text(locale, localized_text_key)?
            }
        };

        Some(Section {
            text,
            screen_position: (self.screen_position.x, self.screen_position.y),
            color: [self.color.x, self.color.y, self.color.z, self.color.w],
            scale: Scale::uniform(self.size.round()),
            z: 1.0,
            ..Section::default()
        })
    }
}
