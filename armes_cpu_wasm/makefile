web_dir := ./web
wasm_dir := $(web_dir)/wasm

wasm_file := $(wasm_dir)/armes_cpu_wasm_bg.wasm

watchfiles = src/*.rs ../armes_cpu_lib/src/*.rs

build:
	@echo "====================================="
	@echo "          🚧 BUILDING  🚧"
	@echo "====================================="
	@wasm-pack build --target web --out-dir $(wasm_dir)
	@echo ">> Updating gitignore"
	@echo "*" > $(wasm_dir)/.gitignore
	@echo "!setup_wasm.js" >> $(wasm_dir)/.gitignore

serve:
	@python3 -m http.server -d $(web_dir)

kill-serve:
	@pkill -f "make serve"

$(wasm_file): $(watchfiles)
	@$(MAKE) build || $(MAKE) build-err

build-require: $(wasm_file)
build-err:
	@echo "\n>> Hit enter when fixed"
	@read

watch:
	@$(MAKE) build
	@echo ""
	@echo "====================================="
	@echo "      👀 WATCHING FOR CHANGES 👀"
	@echo "====================================="
	@echo ""
	@while true; do $(MAKE) build-require -q || $(MAKE) build-require;sleep 0.5; done

watch-serve:
	@$(MAKE) serve &
	@$(MAKE) watch
	@$(MAKE) kill-serve
