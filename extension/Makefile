# Set the default target that `make` will build if no target is specified on the command line.
.DEFAULT_GOAL := test

# Update the `Cargo.lock` file in the `vizsla` directory by running `cargo update`.
vizsla/Cargo.lock:
	cd vizsla && cargo update

# Build the `vizsla.wasm` file in the `vizsla/pkg` directory.
vizsla/pkg/vizsla.wasm: vizsla/Cargo.lock vizsla/Cargo.toml vizsla/src/lib.rs
	cd vizsla && wasm-pack build -t web && cp pkg/vizsla_bg.wasm pkg/vizsla.wasm

# Declare that the `.js` file depends on the `.wasm` file.
vizsla/pkg/vizsla.js: vizsla/pkg/vizsla.wasm

# Create a directory named `wasm`.
wasm:
	mkdir $@

# Copy the `.wasm` file from the `wasm_src/pkg` directory to the `wasm` directory.
wasm/vizsla.wasm: vizsla/pkg/vizsla.wasm | wasm
	cp $< $@

# Copy the `.js` file from the `wasm_src/pkg` directory to the `content_scripts` directory.
content_scripts/vizsla.js: vizsla/pkg/vizsla.js
	cp $< $@

# Create a zip file named `packed.zip` that contains several files.
packed.zip: manifest.json content_scripts/vizsla.js content_scripts/main.js wasm/vizsla.wasm 
	zip $@ $?
# packed.zip: manifest.json assets/fullscreen.svg assets/macro_railroad_ext.css assets/options.svg content_scripts/macro_railroad_ext.js content_scripts/main.js icons/icon48.png icons/icon96.png options/options.html wasm/macro_railroad_ext.wasm
# 	zip $@ $?

# # Declare `clean` as a phony target, which means that `make` will always run its commands, even if a file named `clean` exists.
# .PHONY: clean
# clean:
# 	-rm packed.zip wasm_src/Cargo.lock integration_test/Cargo.lock wasm/macro_railroad_ext.wasm content_scripts/macro_railroad_ext.js
# 	-rm -r wasm_src/pkg
# 	-rmdir wasm
# 	cd wasm_src && cargo clean
# 	cd integration_test && cargo clean
# 	cd chrome_helper && cargo clean

# # Declare `test` as a phony target. The commands associated with this target run tests in the `wasm_src` directory and in the `integration_test` directory.
# .PHONY: test
# test: packed.zip
# 	cd wasm_src && cargo test --release
# 	export RUST_BACKTRACE=1 MACRO_RAILROAD_PACKED="$(abspath $<)"; cargo test --manifest-path integration_test/Cargo.toml -- --test-threads=1

.PHONY: test
test: packed.zip

# # Declare `screenshots` as a phony target. The command associated with this target runs a program that generates screenshots.
# .PHONY: screenshots
# screenshots: packed.zip
# 	export MACRO_RAILROAD_PACKED="$(abspath $<)"; cargo run --manifest-path integration_test/Cargo.toml -- screenshots

# # Declare `playground` as a phony target. The command associated with this target runs a program that starts a playground.
# .PHONY: playground
# playground: packed.zip
# 	export MACRO_RAILROAD_PACKED="$(abspath $<)"; cargo run --manifest-path integration_test/Cargo.toml -- playground
