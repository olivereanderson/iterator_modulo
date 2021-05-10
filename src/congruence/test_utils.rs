use super::Congruence;
pub(crate) fn congruence_pairs<T: Into<Congruence<MODULUS>>, const MODULUS: usize>(
    x: T,
    y: T,
) -> (Congruence<MODULUS>, Congruence<MODULUS>) {
    (x.into(), y.into())
}

pub(crate) fn congruence_triples<T: Into<Congruence<MODULUS>>, const MODULUS: usize>(
    x: T,
    y: T,
    z: T,
) -> (
    Congruence<MODULUS>,
    Congruence<MODULUS>,
    Congruence<MODULUS>,
) {
    (x.into(), y.into(), z.into())
}
