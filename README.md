Rust
----
>Rust feature tests, to include:
1. actix-web
2. chrono
3. futures
4. hyper
5. rocket
6. tide
7. reqwest
8. serde, serde_json
9. tokio
>See: crates.io for these and more dependencies.

Install
-------
>Install with step 1 and validate with steps 2 - 4.
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. rustup --version
3. rustc --version
4. cargo --version

Update
------
1. rustup update && cargo update

VSCode
------
1. rust-analyzer  ( or Rust for VSCODE )
2. crates
3. Even Better TOML

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
1. cargo run --bin actixweb  ( curl http://localhost:7777/now )
2. cargo run --bin hyper     ( curl http://localhost:7878/now )
3. cargo run --bin hyperx    ( curl http://localhost:7979/now )
4. cargo run --bin rocket    ( curl http://localhost:8080/now )
5. cargo run --bin tide      ( curl http://localhost:8081/now )
6. cargo run --bin reqwest
7. cargo run --bin main

Doc
---
1. carg doc --open