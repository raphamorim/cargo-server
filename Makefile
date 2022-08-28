test-simple-wasm-server:
	cargo install --path ./
	cp ./target/release/ou ./examples/simple-wasm-frontend-app
	cd ./examples/simple-wasm-frontend-app && ./ou --port 8123