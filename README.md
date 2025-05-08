# Compiler
This project is a compiler built in Rust. It takes a simple language that includes variable assignments, print statements, arithmetic operations, and for-loops and translates it to MIPS assembly. 

## Topics Learned
Shift-reduce Parsing, Abstract Syntax Trees, Syntax-Directed Translation, Register Allocation and Graph Coloring, Spilling, Loop Optimizations Techniques, and Rust (first Rust project).

## Example 

This source code:
```src
for(i := 0; i < 3; i := i + 1) {
    for(j := 0; j < 3; j := j + 1){
        print(i,j)
    }
}
```
translates to the following MIPS code (with k=1 for graph coloring):
```mips
.text
main:
li $t0, 0
sw $t0, i
LOOPLABEL0:
lw $t8, i
bge $t8, 3, LOOPLABEL1
li $t1, 0
move $s0, $t1
LOOPLABEL2:
bge $s0, 3, LOOPLABEL3
li $v0, 1
lw $t9, i
move $a0, $t9
syscall
li $v0, 4
la $a0, newline
syscall
li $v0, 1
move $a0, $s0
syscall
li $v0, 4
la $a0, newline
syscall
li $t3, 1
add $t4, $s0, $t3
move $s0, $t4
j LOOPLABEL2
LOOPLABEL3:
li $t6, 1
lw $t8, i
add $t7, $t8, $t6
sw $t7, i
j LOOPLABEL0
LOOPLABEL1:
.data
i: .word 0
newline: .asciiz "\n"
```
