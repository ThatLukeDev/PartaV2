/// A ChaCha20 state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChaCha20 {
    pub state: [u32; 16]
}

impl ChaCha20 {
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
}
