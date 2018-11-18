extern crate base64;
extern crate rand;
extern crate rayon;
extern crate x25519_dalek;

use rand::thread_rng;
use rayon::prelude::*;
use std::env;
use x25519_dalek as x25519;

fn main() {
    let prefix = env::args().nth(1).unwrap().to_ascii_lowercase();
    let len = prefix.len() as u64;
    const WITHIN: usize = 10;
    let offsets: u64 = (WITHIN as u64) - len;
    let expected: u64 = 2u64.pow(5).pow(len as u32) / offsets;
    println!(
        "prefix: {}, expect {} trials, Ctrl-C to stop",
        prefix, expected
    );

    // 1M trials takes about 10s on my laptop, so let it run for 1000s
    let _: Vec<bool> = (0..100_000_000)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let private = x25519::generate_secret(&mut rng);
            let public = x25519::generate_public(&private).to_bytes();
            let public_b64 = base64::encode(&public);
            //if public_b64.starts_with(&prefix) {
            if public_b64[..WITHIN].to_ascii_lowercase().contains(&prefix) {
                println!(
                    "private {}, public {}",
                    base64::encode(&private),
                    &public_b64
                );
                true
            } else {
                false
            }
        }).filter(|good| *good)
        .collect();
}
