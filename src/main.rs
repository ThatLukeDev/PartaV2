use partav2::ring::*;
use partav2::rlwe::*;

use std::time::SystemTime;

fn main() {
    const ITERATIONS: i32 = 1000;
    let start = SystemTime::now();

    for _ in 0..ITERATIONS {
        let ring = NegacyclicRing::new(9, 25601);

        let (private1, public1) = RLWE::generate(ring);
        let (key2, public2) = RLWE::respond(ring, public1);
        let key1 = RLWE::parse(ring, private1, public2);

        assert_eq!(key1, key2);
    }

    println!("{} iterations: {}ms/keyshare", ITERATIONS, (start.elapsed().unwrap().as_micros() as f32 / 1000f32 / ITERATIONS as f32 * 10f32).round() / 10f32);
}
