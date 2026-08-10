.PHONY: build release orbien-server orbien web test clean fmt package desktop-sidecar desktop-dev desktop-build

ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
export CARGO_TARGET_DIR := $(ROOT)/target

WEB_DIR := server-ui
DESKTOP_DIR := desktop

web:
	cd $(WEB_DIR) && npm install && npm run build
	@echo "dashboard assets → server/assets"

build:
	cargo build -p orbien-server -p orbien-client

release: web
	cargo build --release -p orbien-server -p orbien-client
	@echo ""
	@echo "artifacts:"
	@ls -lh target/release/orbien-server target/release/orbien

orbien-server:
	./docs/scripts/build-server.sh

orbien:
	cargo build --release -p orbien-client

desktop-sidecar:
	./scripts/prepare-desktop-sidecar.sh

desktop-dev: desktop-sidecar
	cd $(DESKTOP_DIR) && npm install && npm run tauri dev

desktop-build: desktop-sidecar
	cd $(DESKTOP_DIR) && npm install && npm run tauri build

test:
	cargo test --workspace

fmt:
	cargo fmt --all

clean:
	cargo clean
	rm -rf $(WEB_DIR)/node_modules $(WEB_DIR)/dist
	rm -rf $(DESKTOP_DIR)/node_modules $(DESKTOP_DIR)/dist

package: release
	mkdir -p dist
	cp target/release/orbien-server target/release/orbien dist/
	cp -R conf dist/
	@echo "packaged -> dist/"
	@ls -lh dist/
