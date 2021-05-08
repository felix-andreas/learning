.global _start
.text
_start:
    mov rax, 169
    mov rdi, 0xfee1dead
    mov rsi, 0x28121969
    mov rdx, 0x4321fedc
    syscall

    mov rax, 60
    mov rdi, 0
    syscall
