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

Release
-------
1. cargo [ clean ] build --release ( see target/release directory for executables )

Run
---
1. cargo run --bin actix  ( curl http://localhost:7777/now )
2. cargo run --bin hyper  ( curl http://localhost:7878/now )
3. cargo run --bin hyperx ( curl http://localhost:7979/now )
4. cargo run --bin main
5. cargo run --bin reqwest