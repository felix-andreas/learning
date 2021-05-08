.global _start
.text
_start:
    mov rax, 80
    mov rdi, OFFSET filename
    syscall

    mov rax, 60
    mov rdi, 0
    syscall
filename:
    .ascii "/tmp"
    .set filename_length, . - filename
