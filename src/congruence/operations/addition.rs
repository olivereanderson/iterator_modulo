use super::Congruence;
use crate::congruence::cyclic_iterators;
use std::ops::Add;

impl<const MODULUS: usize> Add for Congruence<MODULUS> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let representative = cyclic_iterators::cycle::<MODULUS>()
            .skip(self.representative)
            .skip(other.representative)
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
    fn add() {
        const MODULUS: usize = 3;
        let (x, y) = congruence_pairs::<usize, MODULUS>(1_usize, 7_usize);
        let two = Congruence::<MODULUS>::from(2_usize);
        assert_eq!(two, x + y);
    }

    #[test]
    fn associativity() {
        const MODULUS: usize = 7;
        let (x, y, z) = congruence_triples::<usize, MODULUS>(2_usize, 3_usize, 5_usize);
        assert_eq!(x + (y + z), (x + y) + z);
    }

    #[test]
    fn identity() {
        const MODULUS: usize = 2;
        let x = Congruence::<MODULUS>::from(1_usize);
        assert_eq!(x, x + Congruence::<MODULUS>::ZERO);
        assert_eq!(x, Congruence::<MODULUS>::ZERO + x);
    }

    #[test]
    fn commutativity() {
        const MODULUS: usize = 6;
        let (x, y) = congruence_pairs::<usize, MODULUS>(4_usize, 3_usize);
        assert_eq!(x + y, y + x);
    }
}
