use rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut coin_flips = [true; 100];
    for i in 0..100 {
        coin_flips[i] = rng.gen_bool(0.5)
    }

    println!("{:?}", &coin_flips[0..30]);
}
