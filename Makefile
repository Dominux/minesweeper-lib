test_logic:
	cd ./minesweeper && cargo test && cd ../

wasm_serve:
	cd ./wasm && trunk serve && cd ../

wasm_build:
	cd ./wasm && trunk build && cd ../
