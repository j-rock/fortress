pub mod text_render_request;
pub mod text_warehouse;

pub use fortress_bake::text::GlyphId;
pub use fortress_bake::text::GlyphInfo;
pub use fortress_bake::text::Locale;
pub use fortress_bake::text::NamedText;
pub use fortress_bake::text::PackedGlyphSheet;
pub use fortress_bake::text::TextConfig;
pub use fortress_bake::text::TextSize;

pub use self::text_render_request::TextContent;
pub use self::text_render_request::TextRenderRequest;
pub use self::text_render_request::TextSurface;
pub use self::text_warehouse::TextWarehouse;
