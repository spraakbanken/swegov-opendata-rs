pub use prodash;

pub use prodash::{
    progress::DoOrDiscard,
    unit::{self, Unit},
    NestedProgress,
};

/// A unit for displaying human readable numbers with throughput and progress percentage, and a single decimal place.
pub fn count(name: &'static str) -> Option<Unit> {
    count_with_decimals(name, 1)
}

pub fn count_with_decimals(name: &'static str, _decimals: usize) -> Option<Unit> {
    Some(unit::label_and_mode(
        name,
        unit::display::Mode::with_throughput().and_percentage(),
    ))
}
