.PHONY: build test clean deploy help

help:
	@echo "Emoji Market - Makefile"
	@echo ""
	@echo "Available commands:"
	@echo "  make build    - Build the Solana program"
	@echo "  make test     - Run all tests"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make deploy   - Deploy to local validator"
	@echo "  make check    - Check code without building"
	@echo "  make fmt      - Format code"
	@echo "  make help     - Show this help message"

build:
	@echo "🔨 Building program..."
	@./build.sh

check:
	@echo "🔍 Checking code..."
	@cargo check

fmt:
	@echo "📝 Formatting code..."
	@cargo fmt

clean:
	@echo "🧹 Cleaning..."
	@cargo clean
	@rm -rf target/
	@rm -rf .test-ledger/

test:
	@echo "🧪 Running tests..."
	@./test.sh

deploy:
	@echo "📦 Deploying program..."
	@solana program deploy target/deploy/emojimarket_program.so

