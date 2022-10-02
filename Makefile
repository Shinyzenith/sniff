BINARY := sniff
BUILDFLAGS := --release
TARGET_DIR := /usr/bin
SOURCE_DIR := ./target/release
MAN1_DIR := /usr/share/man/man1
MAN5_DIR := /usr/share/man/man5

all: build

build:
	@cargo build $(BUILDFLAGS)

clean:
	@cargo clean
	@rm ./docs/*.gz

install:
	@find ./docs -type f -iname "*.1.gz" -exec cp {} $(MAN1_DIR) \;
	@find ./docs -type f -iname "*.5.gz" -exec cp {} $(MAN5_DIR) \;
	@mkdir -p $(TARGET_DIR)
	@cp $(SOURCE_DIR)/$(BINARY) $(TARGET_DIR)
	@chmod +x $(TARGET_DIR)/$(BINARY)

uninstall:
	@rm $(TARGET_DIR)/$(BINARY)
	@rm /usr/share/man/man1/$(BINARY).1.gz
	@rm /usr/share/man/man5/$(BINARY).5.gz

.PHONY: build install uninstall
