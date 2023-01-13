all: 
	cargo build --target x86_64-veil.json

release: 
	cargo build --release --target x86_64-veil.json

check: 
	cargo check --target x86_64-veil.json