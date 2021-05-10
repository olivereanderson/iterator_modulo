use std::{iter::Product, usize};
mod addition;
mod multiplication;
mod subtraction;

use super::Congruence;

impl<const MODULUS: usize> Product for Congruence<MODULUS> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x * y)
            .unwrap_or(Congruence::<MODULUS>::from(0_usize))
    }
}

impl<const MODULUS: usize> Congruence<MODULUS> {
    /// Raises self to the power of the exponent
    pub fn pow(self, exponent: usize) -> Self {
        std::iter::repeat(self).take(exponent).product()
    }

    /// Returns the multiplicative inverse wrapped in Some if it exists.
    pub fn inverse(self) -> Option<Self> {
        let one = Congruence::<MODULUS>::from(1_usize);
        (1..MODULUS)
            .into_iter()
            .map(|i| Congruence::<MODULUS>::from(i))
            .find(|x| *x * self == one)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::congruence::test_utils::congruence_pairs;

    #[test]
    fn pow() {
        const MODULUS: usize = 7;
        let x = Congruence::<MODULUS>::from(3_usize);
        let (one, two) = congruence_pairs::<usize, MODULUS>(1, 2);
        assert_eq!(two, x.pow(2));
        // The next two follow from Fermat's little Theorem.
        assert_eq!(one, x.pow(6));
        assert_eq!(x, x.pow(MODULUS));
    }

    #[test]
    fn product() {
        const MODULUS: usize = 8;
        let prod: Congruence<MODULUS> = [2_usize, 3, 5]
            .iter()
            .map(|i| Congruence::<MODULUS>::from(*i))
            .product();
        let six = Congruence::<MODULUS>::from(6_usize);
        assert_eq!(six, prod);
    }

    #[test]
    fn inverse() {
        const MODULUS: usize = 14;
        let (seven, five) = congruence_pairs::<usize, MODULUS>(7, 5);
        let three = Congruence::<MODULUS>::from(3_usize);
        assert_eq!(None, seven.inverse());
        assert_eq!(three, five.inverse().unwrap());
    }
}
