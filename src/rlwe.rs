use crate::ring::NegacyclicRing;

impl NegacyclicRing {
    /// Transforms the input polynomial to depth 1, and acts as the error correction for RLWE.
    ///
    /// ```
    ///# use partav2::ring::*;
    ///# use partav2::rlwe::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(2, 7681).signal(vec![2, 3, 4096, 7661]),
    ///     vec![1, 1, 0, 1]
    /// );
    /// ```
    pub fn signal(&self, poly: Vec<i32>) -> Vec<i32> {
        let mut out = vec![0; self.size().try_into().unwrap()];

        let min_bound = self.modulus / 4;
        let max_bound = 3 * self.modulus / 4;
        for i in 0..poly.len() {
            if poly[i] < min_bound || poly[i] > max_bound {
                out[i] = 1;
            }
            else {
                out[i] = 0;
            }
        }

        out
    }

    /// Combines the key with the signal to generate a shared private key.
    ///
    /// ```
    ///# use partav2::ring::*;
    ///# use partav2::rlwe::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(2, 7681).modulo2(vec![4, 1, 2, 3], vec![1, 2, 3, 4]),
    ///     vec![0, 0, 1, 1]
    /// );
    /// ```
    pub fn modulo2(&self, a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut out = self.add(a, self.mul(b, vec![(self.modulus - 1) / 2]).unwrap()).unwrap();

        for v in &mut out {
            *v %= 2;
        }

        out
    }
}

pub struct RLWE;

#[derive(Clone, Debug, PartialEq)]
pub struct PublicKeypair {
    a: Vec<i32>,
    p: Vec<i32>
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrivateKeypair {
    a: Vec<i32>,
    s: Vec<i32>
}

pub trait KeyshareRLWE {
    fn generate(ring: NegacyclicRing) -> (PrivateKeypair, PublicKeypair);
    fn respond(ring: NegacyclicRing, key: PublicKeypair) -> (Vec<i32>, PublicKeypair);
    fn parse(ring: NegacyclicRing, private: PrivateKeypair, public: PublicKeypair) -> Vec<i32>;
}

pub trait TransmuteBytes {
    fn from_bytes(bytes: Vec<u8>) -> Self;
    fn to_bytes(self) -> Vec<u8>;
}

impl TransmuteBytes for PublicKeypair {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let size = bytes.len() / size_of::<u32>() / 2;
        assert_eq!(size * size_of::<u32>() * 2, bytes.len());

        let mut out1 = vec![0; size];
        let mut out2 = vec![0; size];

        for i in 0..size {
            out1[i] = i32::from_le_bytes(bytes[i * 4 .. i * 4 + 4].try_into().unwrap());
            out2[i] = i32::from_le_bytes(bytes[size * 4 + i * 4 .. size * 4 + i * 4 + 4].try_into().unwrap());
        }

        PublicKeypair {
            a: out1,
            p: out2
        }
    }
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];

        for i in 0..self.a.len() {
            bytes.extend(self.a[i].to_le_bytes());
        }
        for i in 0..self.p.len() {
            bytes.extend(self.p[i].to_le_bytes());
        }

        bytes
    }
}

impl TransmuteBytes for PrivateKeypair {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let size = bytes.len() / size_of::<u32>() / 2;
        assert_eq!(size * size_of::<u32>() * 2, bytes.len());

        let mut out1 = vec![0; size];
        let mut out2 = vec![0; size];

        for i in 0..size {
            out1[i] = i32::from_le_bytes(bytes[i * 4 .. i * 4 + 4].try_into().unwrap());
            out2[i] = i32::from_le_bytes(bytes[size * 4 + i * 4 .. size * 4 + i * 4 + 4].try_into().unwrap());
        }

        PrivateKeypair {
            a: out1,
            s: out2
        }
    }
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];

        for i in 0..self.a.len() {
            bytes.extend(self.a[i].to_le_bytes());
        }
        for i in 0..self.s.len() {
            bytes.extend(self.s[i].to_le_bytes());
        }

        bytes
    }
}

impl KeyshareRLWE for RLWE {
    fn generate(ring: NegacyclicRing) -> (PrivateKeypair, PublicKeypair) {
        let a = ring.rand();

        let s = ring.sample();
        let e = ring.sample();

        let p = ring.add(ring.mul(a.clone(), s.clone()).unwrap(), ring.mul(e, vec![2]).unwrap()).unwrap();

        (
            PrivateKeypair {
                a: a.clone(),
                s
            },
            PublicKeypair {
                a,
                p
            }
        )
    }

    fn respond(ring: NegacyclicRing, key: PublicKeypair) -> (Vec<i32>, PublicKeypair) {
        let sr = ring.sample();
        let er = ring.sample();

        let pr = ring.add(ring.mul(key.a, sr.clone()).unwrap(), ring.mul(er, vec![2]).unwrap()).unwrap();

        let e2r = ring.sample();
        let kr = ring.add(ring.mul(key.p, sr).unwrap(), ring.mul(e2r, vec![2]).unwrap()).unwrap();

        let w = ring.signal(kr.clone());
        let skr = ring.modulo2(kr, w.clone());

        (
            skr,
            PublicKeypair {
                a: w,
                p: pr
            }
        )
    }

    fn parse(ring: NegacyclicRing, private: PrivateKeypair, public: PublicKeypair) -> Vec<i32> {
        let e2i = ring.sample();
        let ki = ring.add(ring.mul(public.p, private.s).unwrap(), ring.mul(e2i, vec![2]).unwrap()).unwrap();

        let ski = ring.modulo2(ki, public.a);

        ski
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_transmute_public_keypair() {
        let ring = NegacyclicRing::new(8, 7681);

        for _ in 0..100 {
            let keypair = PublicKeypair {
                a: ring.sample(),
                p: ring.sample()
            };

            assert_eq!(PublicKeypair::from_bytes(keypair.clone().to_bytes()), keypair);
        }
    }

    #[test]
    fn bytes_transmute_private_keypair() {
        let ring = NegacyclicRing::new(8, 7681);

        for _ in 0..100 {
            let keypair = PrivateKeypair {
                a: ring.sample(),
                s: ring.sample()
            };

            assert_eq!(PrivateKeypair::from_bytes(keypair.clone().to_bytes()), keypair);
        }
    }

    #[test]
    fn key_exchange() {
        for _ in 0..10 {
            let ring = NegacyclicRing::new(9, 25601);

            let (private1, public1) = RLWE::generate(ring);
            let (key2, public2) = RLWE::respond(ring, public1);
            let key1 = RLWE::parse(ring, private1, public2);

            assert_eq!(key1, key2);
        }
    }
}
