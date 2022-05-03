BINARY := sniff
BUILDFLAGS := --release
TARGET_DIR := /usr/bin
SOURCE_DIR := ./target/release

all: build

build:
	@cargo build $(BUILDFLAGS)

install:
	@mkdir -p $(TARGET_DIR)
	@cp $(SOURCE_DIR)/$(BINARY) $(TARGET_DIR)
	@chmod +x $(TARGET_DIR)/$(BINARY)

uninstall:
	@rm $(TARGET_DIR)/$(BINARY)

.PHONY: build install uninstall
