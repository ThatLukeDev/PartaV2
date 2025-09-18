/// A negcyclic polynomial ring type
#[derive(Debug, Clone, Copy)]
struct NegacyclicRing {
    modulus: i32,
    exponent: u32
}

impl NegacyclicRing {
    /// Gives the size of the key, or 2 ^ exponent.
    fn size(&self) -> i32 {
        return (2_i32).pow(self.exponent);
    }
}
