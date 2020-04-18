use rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut coin_flips = [true; 100];
    for i in 0..coin_flips.len() {
        coin_flips[i] = rng.gen_bool(0.5)
    }

    let mut streaks = 0;

    'streak_search: for i in 0..(coin_flips.len() - 6) {
        for j in (i+1)..(i+6) {
            let coin_type = coin_flips[i];
            if coin_flips[j] != coin_type {
                continue 'streak_search
            }
        }
        streaks += 1;
    }

    println!("{:?}", streaks);
}
