.DEFAULT_GOAL := release

NATIVE_EXT_SRC = $(shell find extension -type f)

BUILT_NATIVE_EXT_SRC = $(patsubst extension/%, build/extension/%, $(NATIVE_EXT_SRC))

build/extension/%/: extension/%
	mkdir -p $(@D)
	cp $< $@

build/vizsla.zip: $(BUILT_NATIVE_EXT_SRC)
	@echo "bundling into zipped extension"

.PHONY: release
release: build/vizsla.zip
	@echo "relasing"

.PHONY: clean
clean: 
	@rm -rf build