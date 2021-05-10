use std::{
    iter::Product,
    ops::{Add, Mul, Sub},
    usize,
};

use cyclic_iterators::{cycle, rev_cycle};
mod cyclic_iterators;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Congruence<const MODULUS: usize> {
    representative: usize,
}
impl<const MODULUS: usize> From<usize> for Congruence<MODULUS> {
    fn from(number: usize) -> Self {
        Self {
            representative: (cycle::<MODULUS>().skip(number).next().unwrap_or(0_usize)), // or only happens when MODULUS = 0
        }
    }
}

impl<const MODULUS: usize> From<isize> for Congruence<MODULUS> {
    fn from(number: isize) -> Self {
        let representative = if number < 0 {
            rev_cycle::<MODULUS>()
                .skip(number.abs() as usize)
                .next()
                .unwrap()
        } else {
            cycle::<MODULUS>().skip(number as usize).next().unwrap_or(0)
        };
        Self { representative }
    }
}

impl<const MODULUS: usize> Add for Congruence<MODULUS> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let representative = cycle::<MODULUS>()
            .skip(self.representative)
            .skip(other.representative)
            .next()
            .unwrap();
        Self { representative }
    }
}

impl<const MODULUS: usize> Sub for Congruence<MODULUS> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        // x - y = x + (-y)
        let minus_other_rep = rev_cycle::<MODULUS>()
            .skip(other.representative)
            .next()
            .unwrap();
        let minus_other = Congruence::<MODULUS>::from(minus_other_rep);
        self + minus_other
    }
}

impl<const MODULUS: usize> Mul for Congruence<MODULUS> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let representative = cycle::<MODULUS>()
            .step_by(self.representative)
            .step_by(other.representative)
            .skip(1)
            .next()
            .unwrap();
        Self { representative }
    }
}

impl<const MODULUS: usize> Product for Congruence<MODULUS> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x * y)
            .unwrap_or(Congruence::<MODULUS>::from(0_usize))
    }
}

impl<const MODULUS: usize> Congruence<MODULUS> {
    pub fn pow(self, exponent: usize) -> Self {
        std::iter::repeat(self).take(exponent).product()
    }

    pub fn inverse(self) -> Option<Self> {
        let one = Congruence::<MODULUS>::from(1_usize);
        (1..MODULUS)
            .into_iter()
            .map(|i| Congruence::<MODULUS>::from(i))
            .find(|x| *x * self == one)
    }
}

pub fn is_prime<const MODULUS: usize>() -> bool {
    let one = Congruence::<MODULUS>::from(1_usize);
    let prod: Congruence<MODULUS> = (1..MODULUS)
        .into_iter()
        .map(|i| Congruence::<MODULUS>::from(i))
        .product();
    one == prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_usize() {
        let congruence = Congruence::<3_usize>::from(5_usize);
        assert_eq!(2_usize, congruence.representative);
    }

    #[test]
    fn from_isize() {
        let congruence = Congruence::<3_usize>::from(-5_isize);
        assert_eq!(1_usize, congruence.representative);
    }

    #[test]
    fn zero_to_congruence() {
        const MODULUS: usize = 5;
        let congruence_from_usize = Congruence::<MODULUS>::from(0_usize);
        let congruence_from_isize = Congruence::<MODULUS>::from(0_isize);
        assert_eq!(0_usize, congruence_from_usize.representative);
        assert_eq!(0_usize, congruence_from_isize.representative);
    }

    #[test]
    fn zero_ring() {
        const MODULUS: usize = 0;
        let one = Congruence::<MODULUS>::from(1_usize);
        let zero = Congruence::<MODULUS>::from(0_usize);
        assert_eq!(zero, one);
    }

    #[test]
    fn add() {
        const MODULUS: usize = 3;
        let (x, y) = congruence_pairs::<usize, MODULUS>(1_usize, 2_usize);
        let zero = Congruence::<MODULUS>::from(0_usize);
        assert_eq!(zero, x + y);
    }

    #[test]
    fn sub() {
        const MODULUS: usize = 5;
        let (x, y) = congruence_pairs::<usize, MODULUS>(4, 2);
        assert_eq!(2_usize, (x - y).representative);
        assert_eq!(3_usize, (y - x).representative);
    }

    #[test]
    fn mul() {
        const MODULUS: usize = 5;
        let (x, y) = congruence_pairs::<usize, MODULUS>(0_usize, 4_usize);
        let two = Congruence::<MODULUS>::from(2_usize);
        assert_eq!(two, x * y);
    }

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

    #[test]
    fn const_is_prime() {
        assert!(is_prime::<5_usize>());
    }

    fn congruence_pairs<T: Into<Congruence<MODULUS>>, const MODULUS: usize>(
        x: T,
        y: T,
    ) -> (Congruence<MODULUS>, Congruence<MODULUS>) {
        (x.into(), y.into())
    }
}
