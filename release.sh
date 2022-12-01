RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
#cargo build --release
cp target/release/steam-deck-tools steam-deck-tools
git commit -am "$1"
git push