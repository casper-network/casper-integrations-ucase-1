prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p associate-registrar -p document-notarizer -p test-contracts --target wasm32-unknown-unknown

test-only:
	cargo test --workspace

copy-wasm-file-to-test:
	mkdir -p tests/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all

check-lint: clippy
	cargo fmt --all -- --check

format:
	cargo fmt --all

lint: clippy format
	
clean:
	cargo clean
	rm -rf tests/wasm/*.wasm
