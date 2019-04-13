use crate::{
    app::StatusOr,
    file,
    image::Png,
    render::Texture,
};
use enum_iterator::IntoEnumIterator;
use hashbrown::HashMap;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum NamedTexture {
    SpriteSheet1
}

impl NamedTexture {
    pub fn file_path(self) -> PathBuf {
        let basename = match self {
           NamedTexture::SpriteSheet1 => "sprite_sheet1.png",
        };

        file::util::resource_path("images", basename)
    }

    pub fn all_named_textures() -> NamedTextureEnumIterator {
        Self::into_enum_iter()
    }
}

pub struct NamedTextureManager {
    textures: HashMap<NamedTexture, Texture>,
}

impl NamedTextureManager {
    pub fn new() -> StatusOr<NamedTextureManager> {
        let textures = NamedTexture::all_named_textures()
            .map(|named_texture| {
                let path = named_texture.file_path();
                let image: Png = Png::from_file(&path)?;
                let texture = Texture::new(image, 0)?;
                Ok((named_texture, texture))
            })
            .collect::<StatusOr<HashMap<_, _>>>()?;

        Ok(NamedTextureManager {
            textures
        })
    }

    pub fn texture(&self, named_texture: NamedTexture) -> &Texture {
        self.textures.get(&named_texture).expect("Missing texture!")
    }
}