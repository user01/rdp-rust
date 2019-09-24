
build:
	maturin build

test: clean install
	cargo test --no-default-features
	cargo bench
	pytest -q test_options.py --benchmark-group-by=group

target/wheels/rdp_rust-*-cp37-cp37m-macosx_10_7_x86_64.whl:
	maturin build

install: target/wheels/rdp_rust-*-cp37-cp37m-macosx_10_7_x86_64.whl
	-yes | pip uninstall rdp_rust
	pip install ./target/wheels/rdp_rust-*-cp37-cp37m-macosx_10_7_x86_64.whl

clean:
	rm -rf target/wheels/

all: build install
