A Rust implementation of TOMPAL — Theory of Mind built on Public Announcement Logic, used to predict the answers of 
different Theory of Mind (ToM) levels in the game of Aces and Eights.

## How to use

```bash
cargo run
```

To change the true state or ToM level, modify these lines in `main`:

```rust
let true_state = [2, 0, 0]; // player 0: AA, player 1: 88, player 2: 88
let tom_level = 2;
```

## Reference

Top, J. D., Jonker, C., Verbrugge, R., & de Weerd, H. (2023). Predictive theory of mind models based on public 
announcement logic. In International workshop on dynamic logic (pp. 85-103). Springer Nature Switzerland.
