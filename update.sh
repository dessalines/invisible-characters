git submodule update --remote
cargo run
cargo +nightly fmt
git add invisible_chars
git add src/invisible_chars.rs
git commit "Updating chars."
