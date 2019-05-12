run:
	@python3 cpu.py

crom:
	@python3 romCompiler.py

compile_cpu:
	@pyinstaller --onefile cpu.py
	@rm -r build
	@rm -r cpu.spec

compile:
	@python3 asmCompiler.py program.armes.asm