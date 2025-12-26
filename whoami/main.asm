section .data
home_short db "-h", 0
home_long db "--home", 0
shell_short db "-s", 0
shell_long db "--shell", 0

extern getpwuid
extern strcmp
extern strlen

section .text
global  _start

_start:

	mov  rdi, [rsp]; get argc
	lea  rsi, [rsp + 16]; get the second argument
	call main
	jmp  program_exit

main:
	push rbp
	mov  rbp, rsp
	sub  rsp, 48

	mov [rbp - 8], rdi
	mov [rbp - 16], rsi
	mov QWORD [rbp - 24], 0
	mov QWORD [rbp - 32], 0

	cmp rdi, 1
	je  get_user

	mov rdi, [rbp - 8]
	mov rsi, [rbp - 16]
	lea rdx, [rbp - 24]
	lea rcx, [rbp - 32]

	call check_flag

get_user:
	mov rax, 102
	syscall

	mov  rdi, rax
	call getpwuid
	mov  rbx, rax

	mov  rdi, [rbx]; dereference the pointer
	call strlen
	mov  rdx, rax

	mov  rax, 1
	mov  rdi, 1
	mov  rsi, [rbx]
	;rdx already has the strlen
	syscall

	mov r14, [rbp - 24]
	cmp r14, 1
	je  print_home
	mov r14, [rbp - 32]
	cmp r14, 1
	je  print_shell
	jmp main_end

print_home:
	mov  r12, rbx
	add  r12, 32
	mov  rdi, [r12]
	call strlen
	mov  rdx, rax
	mov  rax, 1
	mov  rdi, 1
	mov  rsi, [r12]
	syscall
	jmp  main_end

print_shell:
	mov  r12, rbx
	add  r12, 40
	mov  rdi, [r12]
	call strlen
	mov  rdx, rax
	mov  rax, 1
	mov  rdi, 1
	mov  rsi, [r12]
	syscall
	jmp  main_end

check_flag:
	push rbp
	mov  rbp, rsp
	sub  rsp, 48

	mov [rbp - 8], rdi
	mov [rbp - 16], rsi
	mov [rbp - 24], rdx
	mov [rbp - 32], rcx

	mov r14, [rbp - 8]; argc
	mov r10, 1
	mov r12, [rbp - 16]

check_loop:
	cmp r10, r14
	je  check_end

	mov  rdi, [r12]
	mov  rsi, home_short
	call strcmp
	cmp  rax, 0
	je   set_home_flag

	mov  rdi, [r12]
	mov  rsi, home_long
	call strcmp
	cmp  rax, 0
	je   set_home_flag

	mov  rdi, [r12]
	mov  rsi, shell_short
	call strcmp
	cmp  rax, 0
	je   set_shell_flag

	mov  rdi, [r12]
	mov  rsi, shell_long
	call strcmp
	cmp  rax, 0
	je   set_shell_flag

	jmp check_end

set_home_flag:
	mov r9, [rbp - 24]
	mov QWORD [r9], 1
	jmp check_end

set_shell_flag:
	mov r9, [rbp - 32]
	mov QWORD [r9], 1
	jmp check_end

check_end:
	add rsp, 48
	pop rbp
	ret

program_exit:
	mov rax, 60
	mov rdi, 0
	syscall

main_end:
	add rsp, 48
	pop rbp
	ret
