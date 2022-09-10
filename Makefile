test-simple-wasm-server:
	cargo install --path ./
	cp ./target/release/ou ./examples/simple-wasm-frontend-app
	cd ./examples/simple-wasm-frontend-app && ./ou --port 8123

# rustc --print target-list
release:
	cargo build --target=aarch64-apple-darwin --release
# 	cargo build --target=x86_64-unknown-linux-musl --release -static
# 	cargo build --target=armv7-unknown-linux-gnueabihf --release

# ou-v0.1.5-aarch64-apple-darwin.tar.xz
install-targets:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-unknown-linux-musl
# 	rustup target add pc-windows-msvc
	rustup target add armv7-unknown-linux-gnueabihf