web_dir := ./www
wasm_dir := $(web_dir)/wasm

wasm_file := $(wasm_dir)/armes_cpu_lib_bg.wasm

build:
	@echo "====================================="
	@echo "           🚧 BUILDING 🚧"
	@echo "====================================="
	@wasm-pack build --target web --out-dir $(wasm_dir)

serve:
	@python3 -m http.server -d $(web_dir)

$(wasm_file): src/*.rs
	@$(MAKE) build

build-require: $(wasm_file)

watch:
	@$(MAKE) build
	@echo ""
	@echo "====================================="
	@echo "      👀 WATCHING FOR CHANGES 👀"
	@echo "====================================="
	@echo ""
	@while true; do $(MAKE) build-require -q || $(MAKE) build-require; sleep 0.5; done
