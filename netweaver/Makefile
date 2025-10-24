.PHONY: all build release clean install test check fmt clippy doc help

CARGO := cargo
INSTALL_PATH := /usr/local/bin

all: build

help:
	@echo "NetWeaver Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build      - Build debug version"
	@echo "  make release    - Build optimized release version"
	@echo "  make install    - Install to $(INSTALL_PATH) (requires sudo)"
	@echo "  make test       - Run all tests"
	@echo "  make check      - Run cargo check"
	@echo "  make fmt        - Format code"
	@echo "  make clippy     - Run clippy lints"
	@echo "  make doc        - Generate documentation"
	@echo "  make clean      - Clean build artifacts"

build:
	$(CARGO) build

release:
	$(CARGO) build --release
	@echo ""
	@echo "Release binary: target/release/netweaver"

install: release
	@if [ "$$(id -u)" -ne 0 ]; then \
		echo "Installation requires root privileges. Use: sudo make install"; \
		exit 1; \
	fi
	install -m 755 target/release/netweaver $(INSTALL_PATH)/netweaver
	@echo "NetWeaver installed to $(INSTALL_PATH)/netweaver"
	@echo ""
	@echo "Optional: Grant network capabilities (Linux only):"
	@echo "  setcap cap_net_raw,cap_net_admin=eip $(INSTALL_PATH)/netweaver"

test:
	$(CARGO) test

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy -- -D warnings

doc:
	$(CARGO) doc --no-deps --open

clean:
	$(CARGO) clean
	@rm -rf target/
	@echo "Build artifacts cleaned"
