PROJECT_NAME = CPM_IF_yaml_validator
BUILD_DIR = target/release
BINARY = $(BUILD_DIR)/$(PROJECT_NAME)
SRC_DIR = src

.PHONY: all build run clean install

all: build

build:
	cargo build --release

run: build
	$(BINARY) example.yaml

clean:
	cargo clean

install: build
	install -m 755 $(BINARY) /usr/local/bin/$(PROJECT_NAME)

uninstall:
	rm -f /usr/local/bin/$(PROJECT_NAME)

