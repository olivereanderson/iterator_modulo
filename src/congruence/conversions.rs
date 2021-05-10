use super::cyclic_iterators;
use super::Congruence;

impl<const MODULUS: usize> From<usize> for Congruence<MODULUS> {
    fn from(number: usize) -> Self {
        Self {
            representative: (cyclic_iterators::cycle::<MODULUS>()
                .skip(number)
                .next()
                .unwrap_or(0_usize)), // or only happens when MODULUS = 0
        }
    }
}

impl<const MODULUS: usize> From<isize> for Congruence<MODULUS> {
    fn from(number: isize) -> Self {
        let representative = if number < 0 {
            cyclic_iterators::rev_cycle::<MODULUS>()
                .skip(number.abs() as usize)
                .next()
                .unwrap()
        } else {
            cyclic_iterators::cycle::<MODULUS>()
                .skip(number as usize)
                .next()
                .unwrap_or(0)
        };
        Self { representative }
    }
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
    fn zero_modulus() {
        const MODULUS: usize = 0;
        let one = Congruence::<MODULUS>::from(1_usize);
        let zero = Congruence::<MODULUS>::from(0_usize);
        assert_eq!(zero, one);
    }
}
