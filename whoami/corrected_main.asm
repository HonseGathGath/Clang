section .data
home_short db "-h", 0
home_long db "--home", 0
shell_short db "-s", 0
shell_long db "--shell", 0
newline db 0x0A, 0
test_msg db "DEBUG: In program", 0x0A, 0

extern getpwuid
extern strcmp
extern strlen
extern write

section .text
global  _start

_start:
	;   Debug: Print test message
	mov rax, 1
	mov rdi, 1
	mov rsi, test_msg
	mov rdx, 13
	syscall

	;   Get argc and argv from stack
	pop rdi; argc
	mov rsi, rsp; argv

	;   Debug: Print argc
	add rdi, '0'
	mov [rsp], dil
	mov rax, 1
	mov rdi, 1
	mov rsi, rsp
	mov rdx, 1
	syscall

	mov rax, 1
	mov rdi, 1
	mov rsi, newline
	mov rdx, 1
	syscall

	;   Reset rdi
	pop rdi; Actually get argc properly
	mov rsi, rsp; argv

	;   Align stack
	and rsp, -16

	;    Call main
	call main

	;   Exit
	mov rax, 60
	xor rdi, rdi
	syscall

main:
	push rbp
	mov  rbp, rsp
	sub  rsp, 48

	;   Save argc and argv
	mov [rbp - 8], rdi
	mov [rbp - 16], rsi

	;   Debug: Print message
	mov rax, 1
	mov rdi, 1
	mov rsi, test_msg
	mov rdx, 13
	syscall

	;   Initialize flags
	mov QWORD [rbp - 24], 0
	mov QWORD [rbp - 32], 0

	;   Check argc
	cmp QWORD [rbp - 8], 1
	je  get_user_only

	;    Call check_flag
	mov  rdi, [rbp - 8]
	mov  rsi, [rbp - 16]
	lea  rdx, [rbp - 24]
	lea  rcx, [rbp - 32]
	call check_flag

get_user_only:
	;   SIMPLIFY: Just print a test message first
	mov rax, 1
	mov rdi, 1
	mov rsi, test_msg
	mov rdx, 13
	syscall

	;   Get UID (syscall 174 for x86_64)
	mov rax, 174
	syscall

	;    Debug: print UID
	push rax
	add  rax, '0'
	mov  [rsp], al
	mov  rax, 1
	mov  rdi, 1
	mov  rsi, rsp
	mov  rdx, 1
	syscall
	pop  rax

	;    Call getpwuid
	mov  rdi, rax
	call getpwuid

	;    Check if NULL
	test rax, rax
	jz   main_end

	mov rbx, rax

	;    Print username
	mov  rdi, [rbx]
	call strlen
	mov  rdx, rax
	mov  rax, 1
	mov  rdi, 1
	mov  rsi, [rbx]
	syscall

	;   Newline
	mov rax, 1
	mov rdi, 1
	mov rsi, newline
	mov rdx, 1
	syscall

	;   Check flags
	cmp QWORD [rbp - 24], 1
	je  print_home

	cmp QWORD [rbp - 32], 1
	je  print_shell

	jmp main_end

print_home:
	;    Print home
	mov  rdi, [rbx + 40]
	call strlen
	mov  rdx, rax
	mov  rax, 1
	mov  rdi, 1
	mov  rsi, [rbx + 40]
	syscall

	mov rax, 1
	mov rdi, 1
	mov rsi, newline
	mov rdx, 1
	syscall
	jmp main_end

print_shell:
	;    Print shell
	mov  rdi, [rbx + 48]
	call strlen
	mov  rdx, rax
	mov  rax, 1
	mov  rdi, 1
	mov  rsi, [rbx + 48]
	syscall

	mov rax, 1
	mov rdi, 1
	mov rsi, newline
	mov rdx, 1
	syscall

main_end:
	add rsp, 48
	pop rbp
	ret

check_flag:
	push rbp
	mov  rbp, rsp
	sub  rsp, 48

	mov [rbp - 8], rdi
	mov [rbp - 16], rsi
	mov [rbp - 24], rdx
	mov [rbp - 32], rcx

	mov r14, [rbp - 8]
	mov r10, 1
	mov r12, [rbp - 16]

check_loop:
	cmp r10, r14
	jge check_end

	mov  rdi, [r12 + r10*8]
	mov  rsi, home_short
	call strcmp
	test rax, rax
	jz   set_home

	mov  rdi, [r12 + r10*8]
	mov  rsi, home_long
	call strcmp
	test rax, rax
	jz   set_home

	mov  rdi, [r12 + r10*8]
	mov  rsi, shell_short
	call strcmp
	test rax, rax
	jz   set_shell

	mov  rdi, [r12 + r10*8]
	mov  rsi, shell_long
	call strcmp
	test rax, rax
	jz   set_shell

	inc r10
	jmp check_loop

set_home:
	mov r9, [rbp - 24]
	mov QWORD [r9], 1
	inc r10
	jmp check_loop

set_shell:
	mov r9, [rbp - 32]
	mov QWORD [r9], 1
	inc r10
	jmp check_loop

check_end:
	add rsp, 48
	pop rbp
	ret
