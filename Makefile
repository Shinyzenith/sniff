BINARY := sniff
BUILDFLAGS := --release
TARGET_DIR := /usr/bin
SOURCE_DIR := ./target/release

all: build

build:
	@cargo build $(BUILDFLAGS)

clean:
	@cargo clean
	@rm *.gz

install:
	@scdoc < $(BINARY).1.scd > $(BINARY).1.gz
	@scdoc < $(BINARY).5.scd > $(BINARY).5.gz
	@mv ./$(BINARY).1.gz /usr/share/man/man1/
	@mv ./$(BINARY).5.gz /usr/share/man/man5/
	@mkdir -p $(TARGET_DIR)
	@cp $(SOURCE_DIR)/$(BINARY) $(TARGET_DIR)
	@chmod +x $(TARGET_DIR)/$(BINARY)

uninstall:
	@rm $(TARGET_DIR)/$(BINARY)
	@rm /usr/share/man/man1/$(BINARY).1.gz
	@rm /usr/share/man/man5/$(BINARY).5.gz

.PHONY: build install uninstall
