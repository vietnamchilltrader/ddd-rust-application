### 1. Install Rust Tools

```bash
# Install Rust toolchain
rustup default stable
rustup toolchain add nightly
rustup component add clippy rustfmt

# Install cargo tools
cargo install cargo-watch        # Auto-reload
cargo install sqlx-cli --features postgres  # DB migrations
cargo install cargo-make         # Task runner
```

### 2. Run 
```bash
# Run docker-compose
cd infra/docker_conpose
docker compose up -d

# Run migrattions
sqlx migrate run

# Run app
cargo make dev
```
