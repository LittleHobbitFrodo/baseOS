;;	init.asm
;;	this file originally belonged to baseOS project
;;		and OS template to build on


;;	this is where does the OS really start



section .text
	extern init
	global _start
	global hang

_start:		;	entrypoint
	cli		;	turn off interrupts

	call init

hang:	;	creates infinite loop to hang the kernel
	cli
	hlt
	jmp hang

