# Armes CPU
A simulated CPU written in python

Complete with its own assembly
***
There are multiple files, but for using it, the main ones are the ones that end in `config.py` & the memory maps (`.mmap`).  
The ROM memory map defines the microinstructions for each instruction  
The RAM memory map defines all the data the program needs & the program itself  
The ROM can be compiled from the `ROM.rommap` file (which is easier to read) using the `romCompiler.py` or `make crom`  
The RAM can be compiled from the `program.armes.asm` using `make compile` or from any file using `python3 asmCompiler.py $FILE`  
***
The `.armes.asm` files are somewhat similar to normal assembly, but still pretty different:  
 * Comments start with `;`
 * Constants have to be defined before their use.  
 They're created like `$new_constant 30`, where the `$` goes before the name, then the name, then a space, the numeric value. They can be used like `LDA $new_constant`
 * Labels or line names are for jumping to that line.  
 They're created like `!new_label CODE`, where the `!` goes before the name, then the name, then a space, the normal code. They're used like `JMP !new_label`
 * Addresses are for referecing lines of code or anything that's in the RAM  
 They're created like `#15`, where the `#` goes before the address, then the address. They're used like `JMP #2` or `LDA #15`

Instructions can be found in the `instconfig.py`, but at the time I'm writing this there are:
 * `LDA` & `STA`: They take a memory address (a constant), and load or store from register A to RAM
 * `LDB` & `STB`: Same thing as `LDA` & `STA` but for register B
 * `ADD`: Adds the numbers from registers A & B and stores the result in register B
 * `OUTB`: Prints to output the value of the B register
 * `OUTR`: Prints to output the value of the RAM in the selected address (constant)
 * `HLT`: Terminates the program
 * `JMP`: Jumps to the label provided
