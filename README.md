# Procedural planet generation

Rust sandbox: ASCII terrain maps in the terminal. **Versions** are split by milestone; older behavior stays runnable as a separate binary.

## Versions

| Version | What it is | Run |
|--------|------------|-----|
| **v0.1** | Two nested loops, **purely random** tile per cell (`rand` only). No noise. | `cargo run --bin v01_random` |
| **v0.2** | **Value noise** height field (`noise2d`) + thresholds → `~` `.` `^`. | `cargo run` (default) |
| *later* | Biomes, images, 3D / planet — TBD. | — |

`Cargo.toml` `version` can follow the same idea (e.g. bump when you consider the default binary “done” for that milestone).

## Run (current default = v0.2)

```bash
cargo run
```

## Run v0.1 only

```bash
cargo run --bin v01_random
```

## Docs

- [`docs/level2.md`](docs/level2.md) — Level 2 noise intro / plan

## Requirements

- [Rust](https://rustup.rs/) (toolchain that supports `edition = "2024"`)
