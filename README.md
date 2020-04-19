# coin-flip-streaks-rs
Test how many streaks of six times Heads or six times Tails are in a streak of 100 coin flips.
Repeat this experiment 200,000 times, and calculate the average result.

We benchmark the efficiency of the implementation using [Criterion](https://docs.rs/criterion). 

## How to benchmark
```shell script
cargo bench
```

## How to change the sample size
If you wish to adjust Criterion's sample size in order to speed up benchmarking, 
then you can do so in `./benches/coin_flip_streaks.rs`:

```rust
//                 Change this ------------|
//                                        vvv
config = Criterion::default().sample_size(100);
```



## How to view benchmark results
Open `./target/criterion/report/index.html` in your browser. For example, in macOS:
```shell script
open ./target/criterion/report/index.html
```
