INSTALL_DIR=../zkwasm-typescript-mini-server/ts/src/application

build:
	wasm-pack build --release --out-name application --out-dir pkg
	wasm-opt -Oz -o $(INSTALL_DIR)/application_bg.wasm pkg/application_bg.wasm
	cp pkg/application.d.ts $(INSTALL_DIR)/application.d.ts
	cp pkg/application_bg.js $(INSTALL_DIR)/application_bg.js
	cp pkg/application_bg.wasm.d.ts $(INSTALL_DIR)/application_bg.wasm.d.ts

clean:
	rm -rf pkg
	rm -rf $(INSTALL_DIR)/application_bg.wasm
	rm -rf $(INSTALL_DIR)/application.d.ts
	rm -rf $(INSTALL_DIR)/application_bg.js
	rm -rf $(INSTALL_DIR)/application_bg.wasm.d.ts
