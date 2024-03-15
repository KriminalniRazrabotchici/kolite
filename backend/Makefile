build:
	cargo build --release

run:
	cargo run

test:
	cargo test -p database -- --test-threads=2
	cargo test
	./test_cleanup.sh

test-database:
	cargo test -p database test_connection -- --test-threads=2
	cargo test -p database test_crud
	./test_cleanup.sh

test-coverage:
	cargo llvm-cov clean --workspace
	cargo llvm-cov --no-report -p database -- --test-threads=2
	cargo llvm-cov --no-report
	cargo llvm-cov report --html
	cargo llvm-cov report --lcov
	./test_cleanup.sh
