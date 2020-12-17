build-local:
	cargo build --release
	cp target/release/libpyrs_reverse_geocoder.so pyrs_reverse_geocoder/pyrs_reverse_geocoder.so
build:
	maturin build

dev:
	maturin develop
	