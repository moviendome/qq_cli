build:
	cargo build --release

install:
	sudo cp target/release/qq /usr/local/bin
