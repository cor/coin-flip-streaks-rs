use rand;
use rand::Rng;
use rand::prelude::ThreadRng;

pub fn experiment() {
    const EXPERIMENT_COUNT: usize = 200_000;

    let mut rng = rand::thread_rng();
    let mut result: [i32; EXPERIMENT_COUNT] = [0; EXPERIMENT_COUNT];
    for i in 0..result.len() {
        result[i] = find_streaks(&mut rng);
    }

    println!("{:?}", average(&result));
}


fn find_streaks(rng: &mut ThreadRng) -> i32 {
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

    return streaks
}

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}