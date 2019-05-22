function padBin(n, p){
    let r = n.toString(2)
    while (r.length < p) {
        r = '0' + r
    }
    return r;
}

function precompile_rom(rom_value, conf){
    let prepared_rom = rom_value
    let idx_change = 0;

    // prepare the rom for the rust compiler
    for(let match of prepared_rom.matchAll(/\${(\w+)}/g)){
        let to_replace = padBin(conf.instructions[match[1]], conf.RAM_addr_length)
        prepared_rom = prepared_rom.slice(0, match.index + idx_change) + to_replace + prepared_rom.slice(match.index + match[0].length + idx_change)
        idx_change += to_replace.length - match[0].length;
    }
    return prepared_rom
}

export function main(wasm){


    wasm.init_wasm()

    async function runCpu(conf, ram, rom){
        wasm.run_cpu(conf, ram, rom)
        globalThis.printNLToCLI()

        console.log("async finished")
    }

    console.log("Hello")
    


    let asm_editor = CodeMirror.fromTextArea(document.getElementById("ASM"), {
        lineNumbers: true,
        mode: "armes_asm"
    })
    let rom_editor = CodeMirror.fromTextArea(document.getElementById("ROM"), {
        lineNumbers: true,
        mode: "rommap",
    })
    let conf_editor = CodeMirror.fromTextArea(document.getElementById("toml"), {
        lineNumbers: true,
        mode: "toml",
    })
    let conf = toml.parse(conf_editor.getValue())
    console.log(conf)
    conf_editor.on("change", ()=> {
        conf = toml.parse(conf_editor.getValue())
        console.log(conf)
    });

    let ram// = wasm.compile_asm(asm_editor.getValue(), conf_editor.getValue())
    let rom// = wasm.compile_rom(precompile_rom(rom_editor.getValue(), conf), conf_editor.getValue())

    function compile_ram(r=true){
        ram = wasm.compile_asm(asm_editor.getValue(), conf_editor.getValue())
        if(r){
            compile_rom(false)
        }
    }

    function compile_rom(r=true){
        rom = wasm.compile_rom(precompile_rom(rom_editor.getValue(), conf), conf_editor.getValue())
        if(r){
            compile_rom(false)
        }
    }

    compile_ram();
    document.getElementById("asmc").onclick = function() {
        //ram = wasm.compile_asm(asm_editor.getValue(), conf_editor.getValue())
        //wasm.print_mem(ram)
        compile_ram()
        globalThis.printToCLI("Compiled assembly succesfully")
        globalThis.printNLToCLI()
        console.log(ram, rom)
    }

    //let inst = JSON.parse(wasm.load_instructions(conf_editor.getValue()))
    document.getElementById("romc").onclick = function() {
        //rom = wasm.compile_rom(precompile_rom(rom_editor.getValue(), conf), conf_editor.getValue())
        //wasm.print_mem(rom)
        compile_rom()
        globalThis.printToCLI("Compiled ROM succesfully")
        globalThis.printNLToCLI()
        console.log(ram, rom)
    }
    let cmd = document.getElementById("cmd");
    cmd.onkeypress = function(e){
        if(e.keyCode == 13 || e.keyCode == 10) {
            //console.log('submit:', document.getElementById("cmd").value)
            //printToCLI(document.getElementById("cmd").value)
            if(cmd.value == "run"){
                compile_ram()
                console.log(ram, rom)
                runCpu(conf_editor.getValue(), ram, rom)
                console.log("async started")
            }
            cmd.value = ""
        }
    }
}