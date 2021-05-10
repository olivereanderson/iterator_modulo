mod conversions;
mod cyclic_iterators;
mod operations;
#[cfg(test)]
mod test_utils;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Type representing Z/MODULUS*Z
pub struct Congruence<const MODULUS: usize> {
    representative: usize,
}

impl<const MODULUS: usize> Congruence<MODULUS> {
    /// The additive identity
    pub const ZERO: Self = Self {
        representative: 0_usize,
    };
    /// The multiplicative identity
    pub const ONE: Self = Self {
        representative: 1_usize,
    };
}
