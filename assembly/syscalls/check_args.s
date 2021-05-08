# prints the num of args and the first 20 chars of the first arg

.global _start
.text
_start:
    add QWORD PTR [rsp], 48
    mov rsi, rsp
    mov rax, 1
    mov rdi, 1
    mov rdx, 1
    syscall

    # print first arg!
    mov rsi, [rsp+16]
    mov rax, 1
    mov rdi, 1
    mov rdx, 20
    syscall

.exit:
    mov rax, 60
    mov rdi, 0
    syscall

message:
    .ascii "Argument passed!\n"
    .set message_size, . - message
