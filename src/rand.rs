use std::time::SystemTime;
use std::process;
use rand::TryRngCore;
use rand::rngs::OsRng;

use crate::chacha::ChaCha20;

/// Random generation
#[derive(Debug)]
pub struct Rand {
    pub seed: [u32; 8],
    pub count: u32,

    pub buffer: Vec<u32>
}

impl Rand {
    /// Randomly seeds a new rand
    ///
    ///```
    ///# use partav2::rand::*;
    /// Rand::new();
    ///```
    pub fn new() -> Self {
        let mut seed = [0u32; 8];

        let epoch: u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let pid = process::id();
        let vec = vec![1, 2, 3, 4];
        let heap = vec.as_ptr() as usize;

        seed[0] = u32::from_le_bytes(epoch.to_le_bytes()[0..4].try_into().unwrap());
        seed[1] = u32::from_le_bytes(epoch.to_le_bytes()[4..8].try_into().unwrap());

        seed[2] = u32::from_le_bytes(heap.to_le_bytes()[0..4].try_into().unwrap());
        seed[3] = u32::from_le_bytes(heap.to_le_bytes()[4..8].try_into().unwrap());

        seed[4] = pid;
        seed[5] = 0x1f2e3d4c;

        let mut rnd: [u8; 8] = [0; 8];
        OsRng.try_fill_bytes(&mut rnd).unwrap();
        seed[6] = u32::from_le_bytes(rnd[0..4].try_into().unwrap());
        seed[7] = u32::from_le_bytes(rnd[4..8].try_into().unwrap());

        Self {
            seed: seed,
            count: 1,

            buffer: vec![]
        }
    }

    /// Random numbers
    ///
    ///```
    ///# use partav2::rand::*;
    /// let mut rnd = Rand::new();
    /// rnd.sample();
    ///```
    pub fn sample(&mut self) -> [u32; 16] {
        let mut nonce: [u32; 3] = [0; 3];

        let epoch: u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let mut rnd: [u8; 4] = [0; 4];
        OsRng.try_fill_bytes(&mut rnd).unwrap();

        nonce[0] = u32::from_le_bytes(epoch.to_le_bytes()[0..4].try_into().unwrap());
        nonce[1] = u32::from_le_bytes(epoch.to_le_bytes()[4..8].try_into().unwrap());
        nonce[2] = u32::from_le_bytes(rnd);

        let chacha = ChaCha20::new(self.seed, self.count, nonce);
        self.count += 1;

        chacha.block().state
    }

    /// Takes a random number from the buffer, regenerating if necessary
    ///
    ///```
    ///# use partav2::rand::*;
    /// let mut rnd = Rand::new();
    /// rnd.next();
    ///```
    pub fn next(&mut self) -> i32 {
        if self.buffer.len() == 0 {
            self.buffer = self.sample().into();
        }

        // We have just checked if empty
        self.buffer.pop().unwrap() as i32
    }
}
