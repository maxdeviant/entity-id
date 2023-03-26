#![doc = include_str!("../README.md")]

pub use entity_id_core::EntityId;

#[cfg(feature = "derive")]
pub use entity_id_derive::EntityId;

/// Private internals needed to support the [`EntityId`](entity_id_derive::EntityId) derive macro.
///
/// **Do not** depend on these directly, or your code is likely to break.
#[doc(hidden)]
#[cfg(feature = "derive")]
pub mod __private {
    pub use entity_id_core::unprefix_id;
}
