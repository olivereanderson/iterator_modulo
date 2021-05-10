use std::{
    iter::{Chain, Cycle, Once, Rev},
    ops::Range,
};

// Returns a cyclic iterator over 0,1,...,MODULUS -1
pub(crate) fn cycle<const MODULUS: usize>() -> Cycle<Range<usize>> {
    (0..MODULUS).into_iter().cycle()
}

// Returns a cyclic iterator over 0, MODULUS -1, MODULUS -2, ...., 1
pub(crate) fn rev_cycle<const MODULUS: usize>() -> Cycle<Rev<Chain<Range<usize>, Once<usize>>>> {
    (1..MODULUS)
        .into_iter()
        .chain(std::iter::once(0))
        .rev()
        .cycle()
}
