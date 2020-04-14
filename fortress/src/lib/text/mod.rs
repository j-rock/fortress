pub mod base_10_char_iterator;
pub mod screen_text_renderer;
pub mod text_renderer;
pub mod text_render_request;
pub mod text_resolver;

pub use fortress_bake::text::GlyphId;
pub use fortress_bake::text::GlyphInfo;
pub use fortress_bake::text::Locale;
pub use fortress_bake::text::NamedText;
pub use fortress_bake::text::PackedGlyphSheet;
pub use fortress_bake::text::RasterSize;
pub use fortress_bake::text::TextConfig;

pub use self::base_10_char_iterator::Base10CharIterator;
pub use self::screen_text_renderer::ScreenTextRenderer;
pub use self::text_renderer::TextRenderer;
pub use self::text_render_request::TextContent;
pub use self::text_render_request::TextRenderRequest;
pub use self::text_resolver::TextResolver;
