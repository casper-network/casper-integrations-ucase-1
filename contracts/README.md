## Make commands
### prepare
Adds wasm to the cargo compilation targets.

### build-contract
Builds the contracts using the nightly toolchain with wasm as the compilation target.

### test-only
Run all tests inside the workspace.

### copy-wasm-file-to-test
Copies the `.wasm` files into `/tests/wasm` folder, where the test framework is set to look for them.

### test
Executes the `build-contract` and `copy-wasm-file-to-test`, then `test-only` commands.

### clippy
Executes the clippy linter on the contract and test codes.

### lint
Runs `clippy` and `format`

### check-lint
Runs `clippy` then executes `fmt` with `--check` enabled. Gives errors for clippy warnings.

### format
Applies formatting to the codes.

### clean
Artifact removal command. (`.wasm` files, `target` folders)

## Rust version
This contract was compiled and ran during development using `1.55.0-nightly (868c702d0 2021-06-30)`

## Casper contract sdk version
casper-types = "1.3.3"
casper-contract = "1.3.3"
casper-engine-test-support = "1.3.3"

### Date 30 September 2021