deps:
	brew install gtk4
	pkg-config --modversion gtk4
	brew install gnome-icon-theme
	cargo build