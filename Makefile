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

.PHONY: 4
4:
	cargo test --manifest-path=day4/Cargo.toml

.PHONY: 5
5:
	cargo test --manifest-path=day5/Cargo.toml

.PHONY: 6
6:
	cargo test --manifest-path=day6/Cargo.toml

.PHONY: 7
7:
	cargo test --manifest-path=day7/Cargo.toml

.PHONY: 8
8:
	cargo test --manifest-path=day8/Cargo.toml

.PHONY: 9
9:
	cargo test --manifest-path=day9/Cargo.toml

.PHONY: 10
10:
	cargo test --manifest-path=day10/Cargo.toml

.PHONY: 11
11:
	cargo test --manifest-path=day11/Cargo.toml

.PHONY: 12
12:
	cargo test --manifest-path=day12/Cargo.toml

.PHONY: all
all: 1 2 3 4 5 6 7 8 9 10 11 12
