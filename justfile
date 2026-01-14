build:
	cargo build --frozen --release

[working-directory: "docs"]
build-docs:
	mdbook build

check:
	cargo clippy --all-features -- -W clippy::pedantic

test: check
	cargo test --all-features

install base_dir="":
	@echo "Installing binary..."
	install -Dm755 target/release/niji -t "{{base_dir}}/usr/bin"

	@echo "Installing license..."
	install -Dm644 LICENSE -t "{{base_dir}}/usr/share/licenses/niji"

	echo "Installing shell completions..."
	install -Dm644 target/release/completions/_niji "{{base_dir}}/usr/share/zsh/site-functions/_niji"
	install -Dm644 target/release/completions/niji.bash "{{base_dir}}/usr/share/bash-completion/completions/niji"
	install -Dm644 target/release/completions/niji.fish "{{base_dir}}/usr/share/fish/vendor_completions.d/niji.fish"

	@echo "Installing built-in modules and themes..."
	mkdir -p "{{base_dir}}/usr/share/niji"
	cp -a assets/modules "{{base_dir}}/usr/share/niji/"
	cp -a assets/themes "{{base_dir}}/usr/share/niji/"
	chown -R root:root "{{base_dir}}/usr/share/niji"
	find "{{base_dir}}/usr/share/niji" -type d -exec chmod 755 {} \;
	find "{{base_dir}}/usr/share/niji" -type f -exec chmod 644 {} \;

	@echo "Installation complete!"
