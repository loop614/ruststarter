RUST := rustc
CARGO := cargo
CARGO_WATCH := cargo watch

run:
	$(CARGO) run 1 2

watch:
	$(CARGO_WATCH) -- cargo run

run_rc:
	$(RUST) src/main.rs && ./main 1 2
