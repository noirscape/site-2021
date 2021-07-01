cargo build --release --target x86_64-unknown-linux-musl
ssh hetzner systemctl stop techpriestsite
scp target/x86_64-unknown-linux-musl/release/techpriestsite hetzner:/opt/noirscape.dev/techpriestsite
ssh hetzner systemctl start techpriestsite
