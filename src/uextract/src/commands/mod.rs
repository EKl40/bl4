//! CLI subcommand implementations

mod find_assets;
mod list_classes;
mod script_objects;
mod texture;

pub use find_assets::find_assets_by_class;
pub use list_classes::list_classes;
pub use script_objects::extract_script_objects;

// Re-export types for API completeness
#[allow(unused_imports)]
pub use script_objects::{ScriptObjectEntry, ScriptObjectsDump};
pub use texture::extract_texture_cmd;
