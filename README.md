# Instructions

Uses Rust's standard build system, `cargo`. See https://rustup.rs for installation.

Build with `cargo build --release`. Binary is called `most_active_cookie`, `cargo build --release` deposits the binary in `$PWD/target`. Can also invoke the binary with `cargo run --release --bin most_active_cookie -- <... ARGS>`.

Run tests with `cargo test`.

The only external dependency is `clap`, for command-line argument parsing. Unlike most popular languages, Rust doesn't have a built-in datetime handling library. Instead, there's a de-facto standard community crate, `chrono`. I avoided using it per the letter of the exercise instructions, so this is more verbose than it otherwise would be.

Finally, `assert_fs`, `assert_cmd`, and `predicates` are included as dev-dependencies, meaning they're present during testing but not linked against the final binary output. These are standard crates used for whole-system integration testing of command-line tools.