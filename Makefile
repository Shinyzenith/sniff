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
	@$(RM) -f ./docs/*.gz
	@$(RM) -f ./docs/*.out

install:
	@find ./docs -type f -iname "*.1.gz" -exec cp {} $(MAN1_DIR) \;
	@find ./docs -type f -iname "*.5.gz" -exec cp {} $(MAN5_DIR) \;
	@mkdir -p $(TARGET_DIR)
	@cp $(SOURCE_DIR)/$(BINARY) $(TARGET_DIR)
	@chmod +x $(TARGET_DIR)/$(BINARY)

uninstall:
	@$(RM) -f $(TARGET_DIR)/$(BINARY)
	@$(RM) -f /usr/share/man/man1/$(BINARY).1.gz
	@$(RM) -f /usr/share/man/man5/$(BINARY).5.gz

.PHONY: build install uninstall
