/// chrono-slots is a library for finding free time slots within a given period,
/// excluding the times of already scheduled events.
pub mod finder;
pub mod periods;

pub use crate::finder::*;
pub use crate::periods::*;
