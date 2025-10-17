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

    /// Combines the ntt-space key with the ntt-space signal to generate a shared private key.
    ///
    /// ```
    ///# use partav2::ring::*;
    ///# use partav2::rlwe::*;
    /// let ring = NegacyclicRing::new(2, 7681);
    /// assert_eq!(
    ///     ring.modulo2(ring.ntt(vec![4, 1, 2, 3]).unwrap(), ring.ntt(vec![1, 2, 3, 4]).unwrap()),
    ///     vec![0, 0, 1, 1]
    /// );
    /// ```
    pub fn modulo2(&self, a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        // let mut out = self.add(a, self.mul(b, vec![(self.modulus - 1) / 2]).unwrap()).unwrap();
        let mut out = self.intt(a.into_iter().zip(b).map(|(a, b)| a + b * (self.modulus - 1) / 2).collect()).unwrap();

        for v in &mut out {
            *v %= 2;
        }

        out
    }
}

pub struct RLWE;

/// A public keypair for the RLWE key exchange, in ntt-space.
#[derive(Clone, Debug, PartialEq)]
pub struct PublicKeypair {
    a: Vec<i32>,
    p: Vec<i32>
}

/// A private keypair for the RLWE key exchange, in ntt-space.
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
        let a = ring.ntt(ring.rand()).unwrap();

        let s = ring.ntt(ring.sample()).unwrap();
        let e = ring.ntt(ring.sample()).unwrap();

        // let p = ring.add(ring.mul(a.clone(), s.clone()).unwrap(), ring.mul(e, vec![2]).unwrap()).unwrap();
        let p = a.clone().into_iter().zip(s.clone()).map(|(a, b)| a * b).zip(e).map(|(z, e)| (z + e * 2) % ring.modulus).collect();

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
        let sr = ring.ntt(ring.sample()).unwrap();
        let er = ring.ntt(ring.sample()).unwrap();

        // let pr = ring.add(ring.mul(key.a, sr.clone()).unwrap(), ring.mul(er, vec![2]).unwrap()).unwrap();
        let pr = key.a.into_iter().zip(sr.clone()).map(|(a, b)| a * b).zip(er).map(|(z, e)| (z + e * 2) % ring.modulus).collect();

        let e2r = ring.ntt(ring.sample()).unwrap();
        // let kr = ring.add(ring.mul(key.p, sr).unwrap(), ring.mul(e2r, vec![2]).unwrap()).unwrap();
        let kr: Vec<i32> = key.p.into_iter().zip(sr).map(|(a, b)| a * b).zip(e2r).map(|(z, e)| (z + e * 2) % ring.modulus).collect();
        let kr_raw = ring.intt(kr.clone()).unwrap();

        let w = ring.ntt(ring.signal(kr_raw)).unwrap();
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
        let e2i = ring.ntt(ring.sample()).unwrap();
        // let ki = ring.add(ring.mul(public.p, private.s).unwrap(), ring.mul(e2i, vec![2]).unwrap()).unwrap();
        let ki = public.p.into_iter().zip(private.s).map(|(a, b)| a * b).zip(e2i).map(|(z, e)| (z + e * 2) % ring.modulus).collect();

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
