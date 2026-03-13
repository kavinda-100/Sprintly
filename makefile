.PHONY: help run build test clean lint

help:
	@echo "Available commands:"
	@echo "  run     - Start the development server with hot reloading"
	@echo "  build   - Build the project"
	@echo "  test    - Run tests"
	@echo "  clean   - Clean target directory"
	@echo "  lint    - Run clippy to check for code issues"

run:
	cargo watch -c -x run
	
build:
	cargo build

test:
	cargo test

clean:
	cargo clean

lint:
	cargo clippy -- -D warnings