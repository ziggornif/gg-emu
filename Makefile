.PHONY: check
check:
	cargo fmt --all
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: test
test:
	cargo test

.PHONY: zexdoc
zexdoc:
	cargo test -p z80 --test zex zexdoc -- --ignored --nocapture
