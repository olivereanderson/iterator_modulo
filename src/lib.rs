mod congruence;
pub use congruence::Congruence;

/// Determine if the constant MODULUS is a prime number
pub fn is_prime<const MODULUS: usize>() -> bool {
    // MODULUS is prime if and only if Z/MODULUS*Z is a field
    // if and only if Z/MODULUS*Z is an integral domain (since Z/MODULUS*Z is a finite ring)
    // if and only if the product of the squares of all non-zero congruence classes is different from zero
    let prod: Congruence<MODULUS> = (1..MODULUS)
        .into_iter()
        .map(|i| Congruence::<MODULUS>::from(i))
        .map(|i| i * i)
        .product();
    Congruence::<MODULUS>::ZERO != prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes() {
        const P1: usize = 2;
        const P2: usize = 3;
        const P3: usize = 5;
        assert!(is_prime::<P1>());
        assert!(is_prime::<P2>());
        assert!(is_prime::<P3>());
    }

    #[test]
    fn composites() {
        const P1: usize = 18;
        const P2: usize = 4;
        const P3: usize = 21;
        // Edge cases:
        const P4: usize = 1;
        const P5: usize = 0;
        assert!(!is_prime::<P1>());
        assert!(!is_prime::<P2>());
        assert!(!is_prime::<P3>());
        assert!(!is_prime::<P4>());
        assert!(!is_prime::<P5>());
    }
}
