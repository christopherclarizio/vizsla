.DEFAULT_GOAL := release

NATIVE_EXT_SRC = $(shell find extension -type f)

WASM_EXT_SRC = $(wildcard vizsla/src/*) vizsla/Cargo.lock vizsla/Cargo.toml

VIZSLA_EXT_BUILD_DIR = build/vizsla/extension/

wasm_ext_src_built: $(WASM_EXT_SRC)
	mkdir -p build/wasm-pack
	wasm-pack build vizsla --target web --no-typescript --out-dir $(shell pwd)/build/wasm-pack
	mkdir -p $(VIZSLA_EXT_BUILD_DIR)wasm
	cp build/wasm-pack/vizsla_bg.wasm $(VIZSLA_EXT_BUILD_DIR)wasm/vizsla_bg.wasm
	cp build/wasm-pack/vizsla.js $(VIZSLA_EXT_BUILD_DIR)wasm/vizsla.js

$(VIZSLA_EXT_BUILD_DIR)%/: extension/%
	mkdir -p $(@D)
	cp $< $@

native_ext_src_built: $(patsubst extension/%, $(VIZSLA_EXT_BUILD_DIR)%, $(NATIVE_EXT_SRC))

ext_built: native_ext_src_built wasm_ext_src_built

ext_packaged: ext_built
	zip build/vizsla/vizsla.zip $(shell find $(VIZSLA_EXT_BUILD_DIR) -type f)

.PHONY: release
release: ext_packaged
	@echo "releasing"

.PHONY: clean
clean: 
	@rm -rf build