section .text
global sys_exit

;; syscall_exit(exit_code);
syscall_exit:
  mov rax, 60
  syscall
