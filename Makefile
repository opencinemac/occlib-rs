.PHONY: format
format:
	-cargo fmt

.PHONY: lint
lint:
	-cargo clippy
	-rustfmt ./src/*.rs --check
	-find . -type f | grep -e \.rs$ | grep -v /target | xargs misspell -error

.PHONY: test
test:
	-cargo test --color=always --no-fail-fast

.PHONY: doc
doc:
	-cargo doc --lib --open
