#[cfg(any(test, feature = "quickcheck"))]
#[macro_use]
extern crate quickcheck;

#[cfg(feature = "quickcheck")]
quickcheck!{}

#[macro_use]
pub mod list;

pub mod map;

pub use list::List;
pub use map::Map;
pub use set::Set;
