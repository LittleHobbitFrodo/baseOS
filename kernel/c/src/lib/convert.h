//	convert.h
//	this file originally belonged to baseOS project
//		an OS template on which to build

#pragma once
#include "./integers.h"


static const char *hex_literals = "0123456789ABCDEF";

void to_string(char *str, size_t val);

void to_stringi(char *str, ssize_t val);

void to_hex(char *str, void *ptr);
void to_hexs(char* str, size_t h);

__attribute__((nonnull(1))) size_t to_int(const char* str);
ssize_t to_inti(const char* str);

#define PTR_CONVERT_LEN ((sizeof(void*) * 2) + 3)
#define U64_HEX_CONVERT_LEN ((sizeof(size_t) * 2) + 1)

#define I64_CONVERT_LEN 20
#define U64_CONVERT_LEN 19
