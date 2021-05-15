cargo build --release --target x86_64-unknown-linux-musl
scp target/x86_64-unknown-linux-musl/release/techpriestsite hetzner:/opt/noirscape.dev/techpriestsite
