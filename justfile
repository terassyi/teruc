
test:	unit-test e2e-test

unit-test:
	cargo test

e2e-test:
	cargo build
	cp target/debug/teruc e2e/
	e2e/test.sh

build:
	cargo build
