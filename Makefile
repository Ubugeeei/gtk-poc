deps:
	brew install gtk4
	pkg-config --modversion gtk4
	cargo build