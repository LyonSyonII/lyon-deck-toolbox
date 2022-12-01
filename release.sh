RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
cp target/release/steam-deck-tools steam-deck-tools