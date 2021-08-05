Rust
----
>Rust feature tests, to include:
1. chrono
2. futures
3. hyper
4. reqwest
5. serde, serde_json
6. tokio

Dependencies
------------
1. See: crates.io

Build
-----
1. cargo clean ( optional )
2. cargo build

Test
----
1. cargo test

Run
---
>Run one of these apps:
1. cargo run --bin hyper
2. cargo run --bin hyperx
3. cargo run --bin main
4. cargo run --bin reqwest

Release
-------
1. cargo [ clean ] build --release ( see target/release directory for executables )
2. ./target/release/[ executable ] ( to run )