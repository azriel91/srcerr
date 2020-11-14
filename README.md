# 🪄 Srcerr

[![Crates.io](https://img.shields.io/crates/v/srcerr.svg)](https://crates.io/crates/srcerr)
![CI](https://github.com/azriel91/srcerr/workflows/CI/badge.svg)
[![Coverage Status](https://codecov.io/gh/azriel91/srcerr/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/srcerr)

User friendly errors from source data.

## Demo

### Suggestions

```rust
error[E1]: `chosen` value `ghi` is invalid.
  --> examples/source_ref_hint.yaml:6:9
   |
 6 | chosen: "ghi"
   |         ^^^^^
   = note: expected one of: `abc`, `def`

help: `chosen` value must come from one of `available` values:
  --> examples/source_ref_hint.yaml:2:1
   |
 2 | available:
   | ---------- hint: first defined here
 3 |   - "abc"
 4 |   - "def"
   |
```

### Long Expressions

```rust
error[E1]: Value `150` is invalid.
  --> /mnt/data/work/github/azriel91/srcerr/examples/long_expr_context.json:1:101
   |
 1 | .. "p":150, ..
   |        ^^^
   |        |
   |        101
   |
   = hint: expected value to be less than 26
```

## Usage

Sample usage can be seen in the [examples](examples).

```
cargo run --example simple
cargo run --example source_ref_hint
cargo run --example source_ref_hint_inline
cargo run --example long_expr_context
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
