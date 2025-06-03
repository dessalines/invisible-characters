git submodule update --remote
cargo run
cargo +nightly fmt
git add invisible-characters
git add src/invisible_chars.rs
git commit -m "Updating chars."
