use super::Congruence;
use crate::congruence::cyclic_iterators;
use std::ops::Sub;

impl<const MODULUS: usize> Sub for Congruence<MODULUS> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        // x - y = x + (-y)
        let minus_other_rep = cyclic_iterators::rev_cycle::<MODULUS>()
            .skip(other.representative)
            .next()
            .unwrap();
        let minus_other = Congruence::<MODULUS>::from(minus_other_rep);
        self + minus_other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::congruence::test_utils::congruence_pairs;
    #[test]
    fn sub() {
        const MODULUS: usize = 5;
        let (x, y) = congruence_pairs::<usize, MODULUS>(4, 2);
        let (two, three) = congruence_pairs::<usize, MODULUS>(2, 3);
        assert_eq!(two, x - y);
        assert_eq!(three, y - x);
    }

    #[test]
    fn additive_inverse() {
        const MODULUS: usize = 3;
        let x = Congruence::<MODULUS>::from(2_usize);
        let identity = Congruence::<MODULUS>::ZERO;
        let minus_x = identity - x;
        assert_eq!(identity, x + minus_x);
        assert_eq!(minus_x + x, identity);
    }
}
