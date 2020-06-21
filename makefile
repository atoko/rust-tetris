build: src/*
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen target\wasm32-unknown-unknown\release\rust-tetris.wasm --out-dir build/wasm --no-modules --no-typescript

copy: build/*
	copy build/wasm/rust-tetris.js build/html
	copy build/wasm/rust-tetris_bg.wasm build/html
