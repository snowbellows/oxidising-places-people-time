// ---- ADDED ----
use nannou::prelude::*;

pub fn map_rng_range<X, Y>(value: X, out_min: Y, out_max: Y) -> Y
where
    X: NumCast + Bounded,
    Y: NumCast,
{
    map_range(value, X::min_value(), X::max_value(), out_min, out_max)
}
// ---------------
