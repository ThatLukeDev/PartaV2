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
}
