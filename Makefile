.DEFAULT_GOAL := all

.PHONY: 1
1: 
	cargo test --manifest-path=day1/Cargo.toml

.PHONY: 2
2: 
	cargo test --manifest-path=day2/Cargo.toml

.PHONY: 3
3: 
	cargo test --manifest-path=day3/Cargo.toml

.PHONY: all
all: 1 2 3