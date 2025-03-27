//	init.h
//	this file originally belonged to baseOS project
//		an OS template on which to build

#pragma once

extern void init();
extern void panic();
extern void hang();






//	variables indicating kernel virtual addresses
	//	defined in linkerscript
extern void* KERNEL_START;
extern void* KERNEL_END;