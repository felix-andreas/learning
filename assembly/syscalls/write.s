.global _start
.text
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, OFFSET message
    mov rdx, OFFSET message_size
    syscall

    mov rax, 60
    mov rdi, 0
    syscall
message:
    .ascii "Hello, world!\n"
    .set message_size, . - message
