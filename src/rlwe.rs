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
