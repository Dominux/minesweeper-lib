test_logic:
	cd ./minesweeper && cargo test && cd ../

wasm_serve:
	cd ./wasm && trunk serve && cd ../
