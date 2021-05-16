use rand::Rng;
use std::iter;
use bitvec::prelude::*;
use sha3::{Digest, Sha3_224};
// For now, assume that a 32-bit integer key is a enough to guarantee eventual matching. We'll
// implement scalable keys later

fn main() {
    let mut rng = rand::thread_rng();
    let things_iter = iter::repeat_with(|| rng.gen::<u32>());
    let message = b"Bob attacks George with a Fire ball";
    let mut hasher = Sha3_224::new();
    let userId: Vec<u8> = vec![15, 14, 65, 200, 43];
    hasher.update(message);

    for (i, n) in (1..).zip(things_iter.take(10)) {
        let mut new_hash = hasher.clone();
        // let hash_key = BitSlice::<Lsb0, _>::from_element(&n);
        new_hash.update(n.to_le_bytes());
        let final_hash = new_hash.finalize();

        println!("* {0:#02} // {1:#010x} // {2:?}", i, n, final_hash);
    }

    println!("Hello, world!");
}