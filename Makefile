.PHONY: zexdoc
zexdoc:
	cargo test -p z80 --test zex zexdoc -- --ignored --nocapture
