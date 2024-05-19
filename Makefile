test:
	cargo test --features="serde"
install-dev:
	cargo install cargo-tarpaulin
	cargo install cargo-watch
	cargo install cargo-readme
generate-docs:
	cargo readme > readme.md
dev-mode-vsc:
	cargo watch -x "tarpaulin --run-types Tests --out Lcov --output-dir coverage; cargo test --doc; cargo doc" # VSCODE - Coverage Gutters



	