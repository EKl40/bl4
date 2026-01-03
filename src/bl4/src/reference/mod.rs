//! Reference data for Borderlands 4 items
//!
//! Hardcoded reference data for game concepts like rarities, elements,
//! weapon types, manufacturers, and gear types. This data is used for
//! display and categorization purposes.

mod element;
mod gear;
mod legendary;
mod manufacturer;
mod rarity;
mod stats;
mod weapon;

pub use element::*;
pub use gear::*;
pub use legendary::*;
pub use manufacturer::*;
pub use rarity::*;
pub use stats::*;
pub use weapon::*;
