import * as elements from "./elements.js";

function padBin(n, p){
    let r = n.toString(2)
    while (r.length < p) {
        r = '0' + r
    }
    return r;
}

export function main(wasm){
    wasm.init_wasm()
    console.log("Hello")
    


    var asm_editor = CodeMirror.fromTextArea(document.getElementById("ASM"), {
        lineNumbers: true,
        mode: "armes_asm"
    })
    var rom_editor = CodeMirror.fromTextArea(document.getElementById("ROM"), {
        lineNumbers: true,
        mode: "rommap",
    })
    var conf_editor = CodeMirror.fromTextArea(document.getElementById("toml"), {
        lineNumbers: true,
        mode: "toml",
    })
    let conf = toml.parse(conf_editor.getValue())
    console.log(conf)
    conf_editor.on("change", ()=> {
        conf = toml.parse(conf_editor.getValue())
        console.log(conf)
    });


    document.getElementById("asmc").onclick = function() {
        let mem = wasm.compile_asm(asm_editor.getValue(), conf_editor.getValue())
        wasm.print_mem(mem)
    }

    //let inst = JSON.parse(wasm.load_instructions(conf_editor.getValue()))
    document.getElementById("romc").onclick = function() {
        let prepared_rom = rom_editor.getValue()
        let idx_change = 0;

        // prepare the rom for the rust compiler
        for(let match of prepared_rom.matchAll(/\${(\w+)}/g)){
            let to_replace = padBin(conf.instructions[match[1]], conf.RAM_addr_length)
            prepared_rom = prepared_rom.slice(0, match.index + idx_change) + to_replace + prepared_rom.slice(match.index + match[0].length + idx_change)
            idx_change += to_replace.length - match[0].length;
        }
        let mem = wasm.compile_rom(prepared_rom, conf_editor.getValue())
        wasm.print_mem(mem)
    }

    document.getElementById("cmd").onkeypress = function(e){
        if(e.keyCode == 13 || e.keyCode == 10) {
            //console.log('submit:', document.getElementById("cmd").value)
            elements.printToCLI(document.getElementById("cmd").value)

            document.getElementById("cmd").value = ""
        }
    }
}