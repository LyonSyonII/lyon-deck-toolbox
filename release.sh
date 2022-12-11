#RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release
cp target/release/lyon-deck-toolbox lyon-deck-toolbox
git commit -am "$1"
git push
