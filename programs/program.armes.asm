$0 1
$1 1
LDA $0
LDB $1
!adding ADD
JO !halting
OUTB
JMP !adding
!halting HLT