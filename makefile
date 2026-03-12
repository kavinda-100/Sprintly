.PHONY: help run build test clean

help:
	@echo "Available commands:"
	@echo "  run     - Start the development server with hot reloading"
	@echo "  build   - Build the project"
	@echo "  test    - Run tests"
	@echo "  clean   - Clean target directory"

run:
	cargo watch -c -x run 
	
build:
	cargo build

test:
	cargo test

clean:
	cargo clean