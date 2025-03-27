//	init.c
//	this file originally belonged to baseOS project
//		an OS template on which to build


#include "./lib/include.h"

void init() {

	//	initialize output structure
	render_init();


	println("hello world!");

}

void panic() {

	renderer.color = col.critical;
	println("PANIC");
	renderer.color = col.white;
	println("\n\nhalting system...");
	hang();

	__builtin_unreachable();
}
