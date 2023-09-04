RUST := rustc
CARGO := cargo

run:
	$(CARGO) run 1 2

run_rc:
	$(RUST) src/main.rs && ./main 1 2
