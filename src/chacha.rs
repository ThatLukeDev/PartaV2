/// A ChaCha20 state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChaCha20 {
    pub state: [u32; 16]
}

impl ChaCha20 {
    /// A new chacha20 state.
    pub fn new(key: [u32; 8], block: u32, nonce: [u32; 3]) -> Self {
        ChaCha20 {
            state: [
                0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
                key[0], key[1], key[2], key[3],
                key[4], key[5], key[6], key[7],
                block, nonce[0], nonce[1], nonce[2]
            ]
        }
    }

    /// ChaCha20 quarter round on a state
    ///
    ///```
    ///# use partav2::chacha::*;
    /// assert_eq!(ChaCha20 {
    ///         state: [
    ///             0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567,
    ///             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ///         ]
    ///     }.quarter_round(0, 1, 2, 3),
    ///     ChaCha20 {
    ///         state: [
    ///             0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb,
    ///             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ///         ]
    ///     }
    /// );
    ///```
    pub fn quarter_round(self, a: usize, b: usize, c: usize, d: usize) -> Self {
        let mut out = self;

        out.state[a] = out.state[a].wrapping_add(out.state[b]);
        out.state[d] ^= out.state[a];
        out.state[d] = (out.state[d] << 16) | (out.state[d] >> (32 - 16));

        out.state[c] = out.state[c].wrapping_add(out.state[d]);
        out.state[b] ^= out.state[c];
        out.state[b] = (out.state[b] << 12) | (out.state[b] >> (32 - 12));

        out.state[a] = out.state[a].wrapping_add(out.state[b]);
        out.state[d] ^= out.state[a];
        out.state[d] = (out.state[d] << 8) | (out.state[d] >> (32 - 8));

        out.state[c] = out.state[c].wrapping_add(out.state[d]);
        out.state[b] ^= out.state[c];
        out.state[b] = (out.state[b] << 7) | (out.state[b] >> (32 - 7));

        out
    }

    /// ChaCha20 block function on a state
    ///
    ///```
    ///# use partav2::chacha::*;
    /// assert_eq!(ChaCha20::new(
    ///         [0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c],
    ///         1,
    ///         [0x09000000, 0x4a000000, 0x00000000]
    ///     ).block(),
    ///     ChaCha20 {
    ///         state: [
    ///             0x837778ab, 0xe238d763, 0xa67ae21e, 0x5950bb2f,
    ///             0xc4f2d0c7, 0xfc62bb2f, 0x8fa018fc, 0x3f5ec7b7,
    ///             0x335271c2, 0xf29489f3, 0xeabda8fc, 0x82e46ebd,
    ///             0xd19c12b4, 0xb04e16de, 0x9e83d0cb, 0x4e3c50a2
    ///         ]
    ///     }
    /// );
    ///```
    pub fn block(self) -> Self {
        let mut out = self;

        for _ in 0..10 {
            out = out.quarter_round(0, 4, 8, 12);
            out = out.quarter_round(1, 5, 9, 13);
            out = out.quarter_round(2, 6, 10, 14);
            out = out.quarter_round(3, 7, 11, 15);
            out = out.quarter_round(0, 5, 10, 15);
            out = out.quarter_round(1, 6, 11, 12);
            out = out.quarter_round(2, 7, 8, 13);
            out = out.quarter_round(3, 4, 9, 14);
        }

        out
    }
}
