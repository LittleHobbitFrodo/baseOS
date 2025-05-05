//	utils.c
//	this file originally belonged to baseOS project
//		an OS template on which to build

#pragma once

#include "./lib/utils.h"

size_t strlen(const char* s) {
	size_t i = 0;
	for (; s[i] != '\0'; i++);
	++i;
	return i;
}

i32 strcmp(const char *str1, const char *str2) {
	while (*str1 && (*str1 == *str2)) {
		str1++;
		str2++;
	}
	return (i32)((u32)*str1 - (u32)*str2);
}


i32 strncmp(const char *str1, const char *str2, size_t n) {
	while (n--) {
		if (*str1 != *str2) {
			return (u32)*str1 - (u32)*str2;
		}
		if (*str1 == '\0') {
			return 0;
		}
		str1++;
		str2++;
	}
	return 0;
}

bool strcmpb(const char* str1, const char* str2) {
	for (size_t i = 0; (str1[i] != '\0') && (str2[i] != '\0'); i++) {
		if (str1[i] != str2[i]) {
			return false;
		}
	}
	return true;
}

bool strncmpb(const char* s1, const char* s2, size_t n) {
	for (size_t i = 0; i < n; i++) {
		if (s1[i] != s2[i]) {
			return false;
		}
	}
	return true;
}

void *memcpy(void *dest, const void *src, size_t n) {
    unsigned char *d = dest;
    const unsigned char *s = src;

    while (n--) {
        *d++ = *s++;
    }
    return dest;
}

void strrev(char* str, size_t len) {
	//	needed by printu function
	size_t start = 0;
	size_t end = len - 1;
	char tmp;
	for (; start < end; start++) {
		tmp = str[start];
		str[start] = str[end];
		str[end] = tmp;
		end--;
	}
}

void *memset(void *s, int c, size_t n) {
	unsigned char *p = s;
	while (n--) {
		*p++ = (unsigned char)c;
	}
	return s;
}