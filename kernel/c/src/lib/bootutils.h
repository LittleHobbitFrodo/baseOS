//	bootutils.h
//	this file originally belonged to baseOS project
//		an OS template on which to build

//	header where are bootloader requests defined

#pragma once
#include "./limine.h"

//	bootloader request for framebuffer (display)
static volatile struct limine_framebuffer_request request_framebuffer = {
	.id = LIMINE_FRAMEBUFFER_REQUEST,
	.revision = 0
};
