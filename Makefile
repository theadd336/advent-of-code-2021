.PHONY: codegen
codegen:
	cargo run --release --bin aoc-codegen

.PHONY: solve
solve:
	cargo run --bin advent-of-code