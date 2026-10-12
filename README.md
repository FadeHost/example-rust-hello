# Rust hello

[![Deploy to FadeHost](https://fadehost.com/deploy-button.svg)](https://laplace.fadehost.com/register?intent=app&repo=https://github.com/FadeHost/example-rust-hello)

A Rust program as small as they come, for FadeHost app hosting: one crate, no
dependencies, prints a line every minute. Push it and FadeHost finds the
Cargo.toml, compiles the crate in release mode on the server and runs the
binary it built.

- Nothing to configure: no build command, no start command
- The compile and the crates it downloads are cached on the app volume, so a
  restart does not compile again
- Replace `src/main.rs` with your bot or worker and keep the rest

```
Cargo.toml      the crate
src/main.rs     the program
```
