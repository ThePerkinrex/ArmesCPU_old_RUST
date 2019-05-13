# Microinstructions
microinst = {
    "RAI": 1 << 0,  # RAM address in
    "RCI": 1 << 1,  # RAM contents in
    "RCO": 1 << 2,  # RAM contents out
    "AI":  1 << 3,  # A register in
    "AO":  1 << 4,  # A register out
    "BI":  1 << 5,  # A register in
    "BO":  1 << 6,  # A register out
    "ICA": 1 << 7,  # Instruction counter add
    "ICI": 1 << 8,  # Instruction counter in
    "ICO": 1 << 9,  # Instruction counter out
    "IRI": 1 << 10, # Instruction regiter in
    "IRO": 1 << 11, # Instruction regiter out
    "OI":  1 << 12, # Output in
    "OSH": 1 << 13, # Output show
    "ADD": 1 << 14, # ALU add
    "HLT": 1 << 15, # Halt
    "FO":  1 << 16, # If overflow flag
    "FZ":  1 << 17, # If zero flag
    "FU":  1 << 18, # If underflow flag
    "SUB": 1 << 19, # ALU sub
}
