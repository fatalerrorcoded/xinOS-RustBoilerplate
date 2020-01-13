TARGET := i386-unknown-none

.PHONY: build

build:
	@RUST_TARGET_PATH=$(shell pwd) \
	xargo build --target=$(TARGET) $(CARGOFLAGS)