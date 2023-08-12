lint:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged

test:
	@cargo test
