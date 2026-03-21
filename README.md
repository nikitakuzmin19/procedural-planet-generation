# Procedural planet generation

ASCII terrain maps in the terminal. **Versions** are split by milestone. Older behavior stays runnable as a separate binary.

## Versions


| Version | What it is                                                                 | Run                          |
| ------- | -------------------------------------------------------------------------- | ---------------------------- |
| **v1**  | Two nested loops, **purely random** tile per cell (`rand` only). No noise. | `cargo run --bin v01_random` |
| **v2**  | **Value noise** height field (`noise2d`) + thresholds → `~` `.` `^`.       | `cargo run` (default)        |
| *later* | Biomes, images, 3D / planet — TBD.                                         | —                            |


`Cargo.toml` `version` can follow the same idea (e.g. bump when you consider the default binary “done” for that milestone).

## Run (current default = v2)

```bash
cargo run
```

## Run v0.1 only

```bash
cargo run --bin v01_random
```

## Requirements

- [Rust](https://rustup.rs/)

