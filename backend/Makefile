build:
	cargo build --release

run:
	cargo run

test:
	cargo test -p database -- --test-threads=1
	cargo test

test-database:
	cargo test -p database test_connection -- --test-threads=1
	cargo test -p database test_crud

test-coverage:
	cargo llvm-cov clean --workspace
	cargo llvm-cov --no-report -p database -- --test-threads=1
	cargo llvm-cov --no-report
	cargo llvm-cov report --html
	cargo llvm-cov report --lcov
