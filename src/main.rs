use iterator_modulo;
fn main() {
    const N: usize = 1597;
    if iterator_modulo::is_prime::<N>() {
        print!("{} is prime! \r\n", N);
    } else {
        print!("{} is not prime! \r\n", N);
    }
}
