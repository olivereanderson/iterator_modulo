use super::Congruence;
use crate::congruence::cyclic_iterators;
use std::ops::Mul;

impl<const MODULUS: usize> Mul for Congruence<MODULUS> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let zero = Congruence::<MODULUS>::ZERO;
        if (self == zero) || (other == zero) {
            return zero;
        }
        let representative = cyclic_iterators::cycle::<MODULUS>()
            .step_by(self.representative)
            .step_by(other.representative)
            .skip(1)
            .next()
            .unwrap();
        Self { representative }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::congruence::test_utils::{congruence_pairs, congruence_triples};

    #[test]
    fn mul() {
        const MODULUS: usize = 5;
        let (x, y) = congruence_pairs::<usize, MODULUS>(3, 4);
        let two = Congruence::<MODULUS>::from(2_usize);
        assert_eq!(two, x * y);
    }

    #[test]
    fn mul_by_zero() {
        const MODULUS: usize = 3;
        let (x, y) = congruence_pairs::<usize, MODULUS>(2, 3);
        let zero = Congruence::<MODULUS>::ZERO;
        assert_eq!(zero, x * y);
        assert_eq!(zero, y * x);
    }

    #[test]
    fn mul_by_one() {
        const MODULUS: usize = 6;
        let one = Congruence::<MODULUS>::ONE;
        let x = Congruence::<MODULUS>::from(23_usize);
        assert_eq!(x, x * one);
        assert_eq!(x, one * x);
    }

    #[test]
    fn mul_is_associative() {
        const MODULUS: usize = 14;
        let (x, y, z) = congruence_triples::<usize, MODULUS>(2, 4, 3);
        assert_eq!(x * (y * z), (x * y) * z);
    }

    #[test]
    fn mul_is_commutative() {
        const MODULUS: usize = 6;
        let (x, y) = congruence_pairs::<usize, MODULUS>(2, 5);
        assert_eq!(x * y, y * x);
    }
}
