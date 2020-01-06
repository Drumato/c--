.intel_syntax noprefix
.global main
main:
  mov rdi, 1
  add rdi, 2
  add rdi, 3
  mov rsi, rdi
  add rsi, 4
  mov rsi, rsi
  add rsi, 5
  mov rsi, rsi
  add rsi, 6
  mov rsi, rsi
  add rsi, 7
  mov rsi, rsi
  add rsi, 8
  mov rsi, rsi
  add rsi, 9
  mov rsi, rsi
  add rsi, 10
  mov rsi, rsi
  mov rax, rsi
  ret

