# Experiment with SurrealDb kv-mem fails sometimes

The code is:
```rust
$ cat -n src/main.rs 
     1  use surrealdb::engine::local::Mem;
     2  use surrealdb::Surreal;
     3
     4  #[tokio::main]
     5  //async fn main() -> Result<(), Box<dyn std::error::Error>> {
     6  async fn main() {
     7      println!("entering");
     8      let db = match Surreal::new::<Mem>(()).await {
     9          Ok(db) => db,
    10          Err(e) => {
    11              println!("Error: {}", e);
    12              return;
    13          }
    14      };
    15
    16      match db.version().await {
    17          Ok(v) => println!("version: {}", v),
    18          Err(e) => println!("version failed: {}", e),
    19      }
    20      
    21      match db.health().await {
    22          Ok(_) => println!("Health check passed"),
    23          Err(e) => println!("Health check failed: {}", e),
    24      }
    25
    26      println!("exiting");
    27  }
```

The failure is:
`called `Result::unwrap()` on an `Err` value: Db(NodeAgent("node task failed and has been logged"))`

```
wink@3900x 24-06-06T00:01:00.030Z:~/prgs/SurrealDB/exper-surrealdb-kv-mem-fails-sometimes (main)
$ while true; do echo RUN;  cargo run; echo DONE; done
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
thread 'tokio-runtime-worker' panicked at /home/wink/.cargo/registry/src/index.crates.io-6f17d22bba15001f/surrealdb-1.5.1/src/api/engine/local/native.rs:237:31:
called `Result::unwrap()` on an `Err` value: Db(NodeAgent("node task failed and has been logged"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
thread 'tokio-runtime-worker' panicked at /home/wink/.cargo/registry/src/index.crates.io-6f17d22bba15001f/surrealdb-1.5.1/src/api/engine/local/native.rs:237:31:
called `Result::unwrap()` on an `Err` value: Db(NodeAgent("node task failed and has been logged"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
thread 'tokio-runtime-worker' panicked at /home/wink/.cargo/registry/src/index.crates.io-6f17d22bba15001f/surrealdb-1.5.1/src/api/engine/local/native.rs:237:31:
called `Result::unwrap()` on an `Err` value: Db(NodeAgent("node task failed and has been logged"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
thread 'tokio-runtime-worker' panicked at /home/wink/.cargo/registry/src/index.crates.io-6f17d22bba15001f/surrealdb-1.5.1/src/api/engine/local/native.rs:237:31:
called `Result::unwrap()` on an `Err` value: Db(NodeAgent("node task failed and has been logged"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
DONE
RUN
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-surrealdb-kv-mem-fails-sometimes`
entering
version: 2.0.0-1.5.1
Health check passed
exiting
thread 'tokio-runtime-worker' panicked at /home/wink/.cargo/registry/src/index.crates.io-6f17d22bba15001f/surrealdb-1.5.1/src/api/engine/local/native.rs:237:31:
called `Result::unwrap()` on an `Err` value: Db(NodeAgent("node task failed and has been logged"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
DONE
RUN
^C
wink@3900x 24-06-06T00:01:08.599Z:~/prgs/SurrealDB/exper-surrealdb-kv-mem-fails-sometimes (main)

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
