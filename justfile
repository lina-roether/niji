name := "niji"

build:
	cargo build --release

[working-directory: "docs"]
build-docs:
	mdbook build

check:
	cargo clippy --all-features -- -W clippy::pedantic

test: check
	cargo test --all-features
