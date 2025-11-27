; x86_64 Exclusively
; Might try to make an aarch_64 compatible version in the future if I feel especially masochistic
; Lowkey might not even work, ts made for linux not windows too 🥀

.global _start
.intel_syntax noprefix

_start:
  mov rax, 1
  mov rdi, 1
  lea rsi, [helloasm]
  mov rdx, [strlen]
  syscall

  mov rax, 60
  mov rdi, 69
  syscall

helloasm:
  .asciz "Hello, Assembly!\n"

strlen:
  equ $ - helloasm
