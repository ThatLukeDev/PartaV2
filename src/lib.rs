#![allow(dead_code)]
use crate::ring::NegacyclicRing;
use crate::rlwe::*;

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
    /// You must make sure that the exponent and modulus form a negacyclic ring
    /// with a 2n-th root of unity
    Custom(u32, i32)
}

impl Security {
    /// Returns the ring from the security preset
    ///
    ///```
    ///# use partav2::ring::*;
    ///# use partav2::*;
    /// assert_eq!(Security::Medium.ring(), NegacyclicRing::new(9, 25601));
    ///```
    pub fn ring(&self) -> NegacyclicRing {
        NegacyclicRing::new(
            match self {
                Security::Medium => 9,
                Security::Custom(pow2, _mod2) => *pow2
            },
            match self {
                Security::Medium => 25601,
                Security::Custom(_pow2, mod2) => *mod2
            }
        )
    }
}

/// Generates a new request keypair for ring learning with errors
///
/// request(security) -> (private, request)
///
///```
/// let (private1, public1) = partav2::request(partav2::Security::Medium);
///```
pub fn request(level: Security) -> (Vec<u8>, Vec<u8>) {
    let ring = level.ring();
    let (private1, public1) = RLWE::generate(ring);

    let mut public = vec![];
    public.extend(ring.modulus.to_le_bytes());
    public.extend(ring.exponent.to_le_bytes());
    public.extend(public1.to_bytes());

    (private1.to_bytes(), public)
}

/// Generates a response keypair for ring learning with errors
///
/// respond(request) -> (key, response)
///
///```
///# let (private1, public1) = partav2::request(partav2::Security::Medium);
/// let (key2, public2) = partav2::respond(public1);
///```
pub fn respond(request: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let ring = NegacyclicRing::new(u32::from_le_bytes(request[4..8].try_into().unwrap()), i32::from_le_bytes(request[0..4].try_into().unwrap()));
    let mut splice = request;
    splice.drain(0..8);

    let public = PublicKeypair::from_bytes(splice);

    let (key1, response) = RLWE::respond(ring, public);

    let mut key = vec![];

    for i in 0..(key1.len() / 8) {
        let mut val: u8 = 0;
        for j in 0..8 {
            val |= (key1[i * 8 + j] << j) as u8;
        }
        key.push(val);
    }

    let mut public = vec![];
    public.extend(ring.modulus.to_le_bytes());
    public.extend(ring.exponent.to_le_bytes());
    public.extend(response.to_bytes());

    (key, public)
}

/// Parses the response keypair
///
/// finalise(rprivate, response) -> key
///
///```
/// let (private1, public1) = partav2::request(partav2::Security::Medium);
/// let (key2, public2) = partav2::respond(public1);
/// let key1 = partav2::finalise(private1, public2);
///
/// assert_eq!(key1, key2);
///```
pub fn finalise(private: Vec<u8>, request: Vec<u8>) -> Vec<u8> {
    let ring = NegacyclicRing::new(u32::from_le_bytes(request[4..8].try_into().unwrap()), i32::from_le_bytes(request[0..4].try_into().unwrap()));
    let mut splice = request;
    splice.drain(0..8);

    let public = PublicKeypair::from_bytes(splice);

    let key1 = RLWE::parse(ring, PrivateKeypair::from_bytes(private), public);

    let mut key = vec![];

    for i in 0..(key1.len() / 8) {
        let mut val: u8 = 0;
        for j in 0..8 {
            val |= (key1[i * 8 + j] << j) as u8;
        }
        key.push(val);
    }

    key
}
