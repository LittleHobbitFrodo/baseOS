;;	init.asm
;;	this file originally belonged to baseOS project
;;		and OS template to build on


;;	this is where does the OS really start



section .text
	extern init
	global _start

_start:		;	entrypoint
	cli		;	turn off interrupts

	call init

hang:	;	creates infinite loop to hang the kernel
	cli
	hlt
	jmp hang








section .text
	extern init
	extern kernel
	global _start
	global hang

_start:
	cli		;	turn off interrupts

	call init	;	call intialization function

hang:	;	creates infinite loop to hang the kernel
	cli		;	do not delete these lines for this function to work properly
	hlt
	jmp hang
