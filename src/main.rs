use coin_flip_streaks::experiment;
use rand::{Rng, RngCore};
use rand::prelude::ThreadRng;

fn main() {
    // experiment();

    let mut data = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);

    // let mut answer = 0b1100_0000_1000_1000_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_1000_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000u128;
    // println!("{:b}", answer);
    // answer >>= 4;
    // println!("{:b}", answer);
}
