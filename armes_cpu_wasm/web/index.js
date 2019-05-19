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


    document.getElementById("asmc").onclick = function() {
        let mem = wasm.compile_asm(asm_editor.getValue(), conf_editor.getValue())
        wasm.print_mem(mem)
    }
}