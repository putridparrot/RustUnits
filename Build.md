# Build

cargo build --verbose
cargo test --verbose

# Deploy to crates.io

Change Cargo.toml package version

cargo login abcdefghijklmnopqrstuvwxyz

where abcdefghijklmnopqrstuvwxyz is you key
	
cargo publish