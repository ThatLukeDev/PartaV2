use partav2::ring::*;
use partav2::rlwe::*;

use std::time::SystemTime;

fn main() {
    const ITERATIONS: i32 = 10;
    let start = SystemTime::now();

    for _ in 0..ITERATIONS {
        let ring = NegacyclicRing::new(9, 25601);

        let (private1, public1) = RLWE::generate(ring);
        let (key2, public2) = RLWE::respond(ring, public1);
        let key1 = RLWE::parse(ring, private1, public2);

        assert_eq!(key1, key2);
    }

    println!("{}ms per keyshare", start.elapsed().unwrap().as_millis() as i32 / ITERATIONS);
}
