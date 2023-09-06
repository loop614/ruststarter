RUST := rustc
CARGO := cargo
CARGO_WATCH := cargo watch

run:
	RUST_BACKTRACE=full $(CARGO) run 1 2

watch:
	$(CARGO_WATCH) -c -w src -x run 

run_rc:
	$(RUST) src/main.rs && ./main 1 2
