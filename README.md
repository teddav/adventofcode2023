# Run

## Latest day

```bash
cargo run
cargo run --release
```

## Any day

```bash
cargo run -- -d 15
cargo run --release -- -d 15
cargo watch -x "run --release -- -d 17" -i src/days/mod.rs
```

## Tests

```bash
cargo watch -x "test day2:: -- --show-output" -i src/days/mod.rs
cargo watch -x "test latest:: -- --show-output" -i src/days/mod.rs
```

## build.rs

Get build.rs output:

```bash
cargo run -vv
cargo watch -x "run -vv" -i src/days/mod.rs
cargo watch -x "run -vv" -i src/days/mod.rs -c
```
