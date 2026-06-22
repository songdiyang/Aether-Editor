pub mod highlighter;
pub mod language;
pub mod theme_mapping;

pub use highlighter::TreeSitterHighlighter;
pub use language::get_language;
pub use theme_mapping::capture_to_textmate_scope;
