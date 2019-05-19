; A fibonacci sequence
;$0 0
;$1 1
;LDB $1
;!fib LDA $0 ; Comment
;ADD
;OUTB
;STB
;JO !hlt
;JMP !fib
;!hlt HLT
; ------------------

$O 255
OUTR $O
HLT