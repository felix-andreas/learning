.global _start
.text
_start:
    mov rax, 79
    mov rdi, OFFSET message
    mov rsi, OFFSET message_size
    syscall

    mov rdi, 1
    mov rsi, rax
    mov rdx, 40
    mov rax, 1
    syscall

    mov rax, 60
    mov rdi, 0
    syscall
message:
    .ascii "Hello, world                                 !\n"
    .set message_size, . - message
