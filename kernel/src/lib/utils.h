//	utils.h
//	this file originally belonged to baseOS project
//		an OS template on which to build

//	header for useful utilities like enabling interrupts and some stdlibc functions


#pragma once

#pragma once
#include "./integers.h"


#define enable_interrupts asm volatile("sti");
#define disable_interrupts asm volatile("cli");

#define comptime_known(var) __builtin_constant_p(var)

#define max(x, max_) ((x > max_)? max_ : x)
#define min(x, min_) ((x < min_)? min_ : x)

#ifdef __GNUC__
	#define likely(x)   __builtin_expect(!!(x), 1)
	#define unlikely(x) __builtin_expect(!!(x), 0)
#else
	#define likely(x)   (x)
	#define unlikely(x) (x)
#endif



//	string function
i32 strcmp(const char *s1, const char *s2);
size_t strlen(const char *s);
bool strcmpb(const char *s1, const char *s2);
i32 strncmp(const char* s1, const char* s2, size_t n);
bool strncmpb(const char* s1, const char* s2, size_t n);

//	reserved by the to_string function (in convert.c)
void strrev(char *str, size_t len);


//	memory functions
void *memcpy(void *dest, const void *src, size_t n);
void* memset(void* s, int c, size_t n);


///	io function wrappers
__attribute__((always_inline))
inline byte inb(u16 port) {
	byte ret;
	asm volatile("in %b0, %1" : "=a"(ret) : "Nd"(port));
	return ret;
}

__attribute__((always_inline))
inline void outb(u16 port, u8 data) {asm volatile("out %b1, %0" :: "a"(data), "Nd"(port));}

__attribute__((always_inline))
inline void outw(u16 port, u16 data) {asm volatile("out %w1, %0" :: "a"(data), "Nd"(port));}

__attribute__((always_inline))
inline u16 inw(u16 port) {
	u16 ret;
	asm volatile("in %w0, %1" : "=a"(ret) : "Nd"(port));
	return ret;
}

__attribute__((always_inline))
inline void outd(u16 port, u32 data) {asm volatile("out %d1, %0" :: "a"(data), "Nd"(port));}

__attribute__((always_inline))
inline u32 ind(u16 port) {
	u32 ret;
	asm volatile("in %d0, %1" : "=a"(ret) : "Nd"(port));
	return ret;
}

__attribute__((always_inline))
inline void outq(u16 port, u64 data) {asm volatile("out %q1, %0" :: "a"(data), "Nd"(port));}

__attribute__((always_inline))
inline u64 inq(u16 port) {
	u64 ret;
	asm volatile("in %q0, %1" : "=a"(ret) : "Nd"(port));
	return ret;
}

