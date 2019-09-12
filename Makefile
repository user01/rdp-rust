
build:
	maturin build

test:
	cargo test --no-default-features

target/wheels/string_sum-0.1.0-cp37-cp37m-macosx_10_7_x86_64.whl:
	maturin build

install: target/wheels/string_sum-0.1.0-cp37-cp37m-macosx_10_7_x86_64.whl
	-yes | pip uninstall string-sum
	pip install ./target/wheels/string_sum-0.1.0-cp37-cp37m-macosx_10_7_x86_64.whl
