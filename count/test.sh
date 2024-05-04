# Watches the Rust files that are defined and runs tests.
watchman-make -p 'src/**/*.rs' --run "cargo test"
#cargo test --bin bubble
