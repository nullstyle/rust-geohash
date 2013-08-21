all: build test
clean:
	rm -rf build

BUILD=rustc
TEST=rustc --test
BUILD_DIR=build
RUSTARGS=--out-dir $(BUILD_DIR)

libgeohash:
	$(BUILD) $(RUSTARGS) lib.rs

test:
	$(TEST) $(RUSTARGS) lib.rs
	./$(BUILD_DIR)/lib

build: libgeohash
	
