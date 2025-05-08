.text
main:
li $t0, 0
move $s0, $t0
LOOPLABEL0:
bge $s0, 3, LOOPLABEL1
li $t1, 0
sw $t1, j
LOOPLABEL2:
lw $t8, j
bge $t8, 3, LOOPLABEL3
li $v0, 1
move $a0, $s0
syscall
li $v0, 4
la $a0, newline
syscall
li $v0, 1
lw $t9, j
move $a0, $t9
syscall
li $v0, 4
la $a0, newline
syscall
li $t3, 1
lw $t8, j
add $t4, $t8, $t3
sw $t4, j
j LOOPLABEL2
LOOPLABEL3:
li $t6, 1
add $t7, $s0, $t6
move $s0, $t7
j LOOPLABEL0
LOOPLABEL1:
.data
j: .word 0
newline: .asciiz "\n"