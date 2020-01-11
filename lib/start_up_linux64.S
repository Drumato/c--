_start:
  call main
  movq %rax, %rdi
  movq $60, %rax
  syscall
