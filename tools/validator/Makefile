# SPDX-License-Identifier: MIT
# 
# MIT License
# 
# © 2024 Nathan Dautenhahn & Serenitix LLC
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# Info for the Rust build tool: name must match Cargo.toml
PROJECT_NAME 	:= cpm_if_yaml_validator
BUILD_DIR 		:= target/release
BINARY 			:= $(BUILD_DIR)/$(PROJECT_NAME)
SRC_DIR 		:= src

# Default schema file, bump this or set the env variable
SCHEMA 			:= ../../specification/cpm_if_schema_v1.3.json

# Example file to show the IF, can be overwritten by ENV variable
IF_FILE			:= ../../examples/linux_2.yaml

.PHONY: all build run test clean install

all: build

example: build
	$(BINARY) $(SCHEMA) $(IF_FILE)

test: build
	cargo test

build:
	cargo build --release

clean:
	cargo clean

install: build
	install -m 755 $(BINARY) /usr/local/bin/$(PROJECT_NAME)

uninstall:
	rm -f /usr/local/bin/$(PROJECT_NAME)

