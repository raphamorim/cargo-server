test-file-server:
	cargo install --path ./
	cd ./examples/simple-wasm-frontend-app && \
		echo ">>> Check help" && \
		cargo server --help && \
		echo ">>> Check version" && \
		cargo server --version && \
		echo ">>> Open with port and" && \
		cargo server --port 8123 --open

test-route:
	cargo install --path ./
	cargo server \
		--route '/users' \
		--json '{"users":[{"data":{"userId":"1","givenName":"Raphael","country":"br"}},{"data":{"userId":"2","givenName":"Emil","country":"se"}}]}' \
		--port 8123

test-all-routes:
	cargo install --path ./
	cargo server --all-routes --json '{"id": "1"}' --port 8123

test-route-with-params:
	cargo install --path ./
	cargo server \
		--route '/users/:userId' \
		--json '{"data":{"userId":"{!0}","givenName":"Raphael","country":"se"}}' \
		--port 8123

lint:
	cargo fmt -- --check --color always
	cargo clippy --all-targets -- -D warnings

# rustc --print target-list
release:
# 	cargo build --target=aarch64-apple-darwin --release
# 	cargo build --target=x86_64-unknown-linux-musl --release
	cargo build --target=armv7-unknown-linux-musleabihf --release

# ou-v0.1.5-aarch64-apple-darwin.tar.xz
install-targets:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-unknown-linux-musl
# 	rustup target add pc-windows-msvc
	rustup target add armv7-unknown-linux-musleabihf