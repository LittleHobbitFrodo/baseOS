//	render.c
//	this file originally belonged to baseOS project
//		an OS template on which to build


#pragma once
#include "./lib/render.h"
#include "./lib/init.h"
#include "./lib/bootutils.h"
#include "./lib/limine.h"
#include "./lib/convert.h"

void render_init() {
	//	initializes renderer

	screen_init();

	screen_flush();

	renderer.line = (renderer.column = 0);
	renderer.space_between_lines = RENDERER_SPACE_BETWEEN_LINES;
	renderer.fb = 0;
	renderer.color = col.white;
}

//	initialization functions
void screen_init() {

	if (request_framebuffer.response == NULL) {
		hang();
	}

	if (request_framebuffer.response->framebuffer_count < 1) {
		hang();
	}

	struct limine_framebuffer* fb = request_framebuffer.response->framebuffers[0];

	if (fb->memory_model != LIMINE_FRAMEBUFFER_RGB) {
		hang();
	}

	if (fb->address == NULL) {
		hang();
	}

	screen.count = request_framebuffer.response->framebuffer_count;
	screen.bpp = fb->bpp;
	screen.address = (union color_t*)fb->address;
	screen.w = fb->width;
	screen.h = fb->height;

}

void screen_flush() {
	size_t size = screen.w * screen.h;
	for (size_t i = 0; i < size; i++) {
		screen.address[i].uint = 0;
	}

	renderer.column = 0;
	renderer.line = 0;
}


//	output functions
void print(const char *s) {
	//	prints text with no linebreak
	for (size_t i = 0; s[i] != '\0'; i++) {
		render(s[i]);
	}
}

void println(const char *s) {
	//	prints text with linebreak
	for (size_t i = 0; s[i] != '\0'; i++) {
		render(s[i]);
	}
	endl();
}

void printi(i64 i) {
	//	converts signed number and prints it
	char num[I64_CONVERT_LEN];
	to_stringi((char*)&num, i);
	print((const char*) &num);
}

void printu(const u64 u) {
	//	converts unsigned number and prints it
	char num[U64_CONVERT_LEN];
	to_string((char *) &num, u);
	print((const char *) &num);
}

void printp(void *p) {
	//	prints pointer (in hex)
	if (p == NULL) {
		print("NULL");
		return;
	}
	char num[PTR_CONVERT_LEN];
	to_hex((char *) num, p);
	print((const char *) &num);
}

void printh(size_t h) {
	//	prints number in hex
	char num[U64_HEX_CONVERT_LEN];
	to_hexs((char*)&num, h);
	print((const char*)&num);
}

void printb(size_t bin) {
	//	prints data in binary
	for (ssize_t i = (sizeof(size_t) * 8) - 1; i >= 0; i--) {
		render('0' + ((bin >> i) & 1));
	}
}

void printn(const char* str, size_t n) {
	//	prints N characters of string
	for (size_t i = 0; i < n; i++) {
		render(str[i]);
	}
}


void render(const char c) {
	//	prints character
	if ((c < ' ') && (c != '\t') && (c != '\n')) {
		return;
	}
	switch (c) {
		case '\n': {
			endl();
			break;
		}
		case ' ': {
			renderer.column++;
			if (renderer.column >= screen.w) {
				endl();
			}
			break;
		}
		case '\t': {
			tab();
			break;
		}
		default: {
			u8 actual = c - RENDERER_FONT_PLACE_SUB;
			union color_t *ptr = (union color_t*)(screen.address + ((renderer.line * screen.w * (8 + renderer.space_between_lines))) + (renderer.column * 8));
			u8 *fnt;
			for (u16 i = 0; i < 8; i++) {
				fnt = font.table[actual];
				for (u16 ii = 0; ii < 8; ii++) {
					(ptr + (i * screen.w) + (8 - ii))->uint = renderer.color.uint * ((fnt[i] >> ii) & 1);
				}
			}

			renderer.column++;
			if ((renderer.column * 8) >= screen.w) {
				endl();
			}
		}
	}
}











