# clang from nix doesn't seem to support wasm32 even though it's listed in llc --version
add.wasm: add.c
	clang -O3 -fuse-ld=wasm-ld --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -o $@ $^