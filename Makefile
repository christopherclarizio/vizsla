.DEFAULT_GOAL := release

NATIVE_EXT_SRC = $(shell find extension -type f)

BUILT_NATIVE_EXT_SRC = $(patsubst extension/%, build/extension/%, $(NATIVE_EXT_SRC))

WASM_EXT_SRC = $(wildcard vizsla/src/*) vizsla/Cargo.lock vizsla/Cargo.toml

wasm_ext_src_built: $(WASM_EXT_SRC)
	mkdir -p build/wasm-pack
	wasm-pack build vizsla --target web --no-typescript --out-dir $(shell pwd)/build/wasm-pack
	mkdir -p build/extension/wasm
	cp build/wasm-pack/vizsla_bg.wasm build/extension/wasm/vizsla_bg.wasm
	cp build/wasm-pack/vizsla.js build/extension/wasm/vizsla.js

build/extension/%/: extension/%
	mkdir -p $(@D)
	cp $< $@

build/vizsla.zip: $(BUILT_NATIVE_EXT_SRC) wasm_ext_src_built
	@echo "bundling into zipped extension"

.PHONY: release
release: build/vizsla.zip
	@echo "relasing"

.PHONY: clean
clean: 
	@rm -rf build