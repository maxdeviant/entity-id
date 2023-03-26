#![doc = include_str!("../README.md")]

pub use entity_id_core::EntityId;

#[cfg(feature = "derive")]
pub use entity_id_derive::EntityId;

#[doc(hidden)]
#[cfg(feature = "derive")]
pub mod __private {
    pub use entity_id_core::unprefix_id;
}
