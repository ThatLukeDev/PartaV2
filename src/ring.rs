/// A negcyclic polynomial ring type
#[derive(Debug, Clone, Copy)]
pub struct NegacyclicRing {
    modulus: i32,
    exponent: u32
}

impl NegacyclicRing {
    /// Creates a new negacyclic ring (form X^n + 1)_q from n and q.
    pub fn new(n: u32, q: i32) -> Self {
        Self {
            modulus: q,
            exponent: n
        }
    }

    /// Gives the size of the key, or 2 ^ exponent.
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(4, 23).size(),
    ///     16 // 2 ^ 4
    /// )
    /// ```
    pub fn size(&self) -> i32 {
        return (2_i32).pow(self.exponent);
    }

    /// Returns x ^ y with respect to the Negacyclic ring.
    ///
    /// Does not support a negative exponent.
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(4, 23).power(3, 4),
    ///     12 // 3 ^ 4 mod 23
    /// )
    /// ```
    pub fn power(&self, x: i32, y: i32) -> i32 {
        match y {
            ..0 => panic!("termPower: y: Argument less than zero. Negative exponents are prohibited."),
            0 => 1,
            0.. => {
                let mut working: i32 = 1;

                let mut base: i32 = x % self.modulus;
                let mut exp: i32 = y;
                while exp > 0 {
                    if exp % 2 == 1 {
                        working = (working * base) % self.modulus;
                    }
                    exp /= 2;
                    base = (base * base) % self.modulus;
                }

                working
            }
        }
    }

    /// Returns the primitive nth root of unity (if one exists).
    ///
    /// z is the primitive nth root of unity where:
    /// - z^n = 1 mod q
    /// - z^k != 1 mod q for all k < n
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(4, 7681).primitiventhunity(),
    ///     Some(3383)
    /// );
    /// ```
    pub fn primitiventhunity(&self) -> Option<i32> {
        for root in 0..self.modulus {
            if self.power(root, self.exponent.try_into().unwrap()) == 1 {
                let mut taken = false;
                for k in 1..self.exponent.try_into().unwrap() {
                    if self.power(root, k) == 1 {
                        taken = true;
                    }
                }
                if !taken {
                    return Some(root);
                }
            }
        }

        None
    }

    /// Returns the primitive 2nth root of unity (if one exists).
    ///
    /// z is the primitive 2nth root of unity where:
    /// - z^2 = v mod q
    /// - z^n = 1 mod q
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(4, 7681).primitive2nthunity(),
    ///     Some(1925)
    /// );
    /// ```
    pub fn primitive2nthunity(&self) -> Option<i32> {
        let nthunity = self.primitiventhunity().unwrap();

        for root in 0..self.modulus {
            if self.power(root, 2) == nthunity && self.power(root, self.exponent.try_into().unwrap()) == self.modulus - 1 {
                return Some(root);
            }
        }

        None
    }

    /// Pads a polynomial to the correct length in a negacyclic ring.
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(3, 7681).pad(vec![1, 2, 3]),
    ///     vec![1, 2, 3, 0, 0, 0, 0, 0] // 2 ^ 3 = 8
    /// );
    /// ```
    pub fn pad(&self, val: Vec<i32>) -> Vec<i32> {
        let mut out = val;

        out.resize(self.size().try_into().unwrap(), 0);

        out
    }

    /// Provides the bit reversal for NTT.
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::bit_reverse(7, 4),
    ///     14 // 7: 0b0111 -> 14: 0b1110
    /// );
    /// ```
    pub fn bit_reverse(x: i32, k: i32) -> i32 {
        let mask = (1 << k) - 1;

        let mut v = x & mask;
        let mut out = 0;

        for _ in 0..k {
            out <<= 1;
            out |= v & 1;
            v >>= 1;
        }

        out
    }

    /// Provides the inverse of a number with respect to the modulus.
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(3, 7681).inverse(14),
    ///     Some(1646)
    /// );
    /// assert_eq!(
    ///     (1646 * 14) % 7681,
    ///     1
    /// );
    /// ```
    pub fn inverse(&self, x: i32) -> Option<i32> {
        for i in 1..self.modulus {
            if (i * x) % self.modulus == 1 {
                return Some(i);
            }
        }

        None
    }

    /// Number theoretic transform in the negacyclic ring.
    ///
    /// ```
    ///# use partav2::ring::*;
    /// assert_eq!(
    ///     NegacyclicRing::new(4, 7681).ntt(vec![1, 2, 3, 4]),
    ///     Some(vec![1467, 2807, 3471, 7621])
    /// );
    /// ```
    pub fn ntt(&self, val: Vec<i32>) -> Option<Vec<i32>> {
        let mut out = self.pad(val);

        let rootunity = self.primitive2nthunity().unwrap();
        let k: usize = self.exponent.try_into().unwrap();
        let q = self.modulus;
        let n: usize = self.size().try_into().unwrap();

        let mut t: usize = n;
        let mut m: usize = 1;
        while m < n {
            t /= 2;

            for i in 0..m {
                let j1 = 2 * i * t;
                let j2 = j1 + t;
                let s = self.power(rootunity, NegacyclicRing::bit_reverse((m + i).try_into().unwrap(), k.try_into().unwrap()));

                for j in j1..j2 {
                    let u = out[j];
                    let v = out[j + t] * s;
                    out[j] = (u + v) % q;
                    out[j + t] = (u - v + q) % q;
                }
            }

            m *= 2;
        }

        let mut ordered = vec![];

        for i in 0..n {
            ordered[<i32 as TryInto<usize>>::try_into(NegacyclicRing::bit_reverse(i.try_into().unwrap(), k.try_into().unwrap())).unwrap()] = out[i];
        }

        Some(ordered)
    }
}
