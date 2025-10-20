#![allow(dead_code)]
use crate::ring::NegacyclicRing;

/// Contains representation of negacyclic polynomial rings, and utility functions
pub mod ring;

/// An implementation of ChaCha20
pub mod chacha;

/// Random
pub mod rand;

/// Ring learning with errors key exchange
pub mod rlwe;

/// The parameters for the ring learning with errors key exchange
pub enum Security {
    /// Equivelent to 128-bit RSA
    Medium,
    /// Equivelent to 256-bit RSA
    High,
    /// You must make sure that the exponent and modulus form a negacyclic ring with a 2n-th root
    /// of unity
    Custom(u32, i32)
}

impl Security {
    /// Returns the ring from the security preset
    ///
    ///```
    ///# use partav2::ring::*;
    ///# use partav2::*;
    /// assert_eq!(Security::Medium.ring(), NegacyclicRing::new(9, 25601));
    /// assert_eq!(Security::High.ring(), NegacyclicRing::new(10, 40961));
    ///```
    pub fn ring(&self) -> NegacyclicRing {
        NegacyclicRing::new(
            match self {
                Security::Medium => 9,
                Security::High => 10,
                Security::Custom(pow2, _mod2) => *pow2
            },
            match self {
                Security::Medium => 25601,
                Security::High => 40961,
                Security::Custom(_pow2, mod2) => *mod2
            }
        )
    }
}
